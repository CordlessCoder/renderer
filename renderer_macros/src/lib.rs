use proc_macro::TokenStream;
use winnow::{ascii, combinator, prelude::*, token};

fn generate_combination_indices(pool_size: usize, items: usize, mut cb: impl FnMut(&[usize])) {
    if items == 0 {
        return;
    };
    assert!(
        pool_size != 0,
        "Cannot draw combinations from a pool of 0 items"
    );
    let mut indices = vec![0; items];
    'outer: loop {
        cb(&indices);
        if indices.iter().all(|idx| *idx == pool_size - 1) {
            break;
        }

        // Handle values that we need to carry
        let mut carry = true;
        for index in indices.iter_mut().rev() {
            if !carry {
                continue 'outer;
            }
            *index += 1;
            carry = *index >= pool_size;
            if carry {
                *index = 0;
            }
        }
    }
}

#[proc_macro]
pub fn swizzle(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input: String = input
        .lines()
        .map(|line| {
            line.find("//")
                .map(|end| &line[..end])
                .unwrap_or(line)
                .to_string()
                + "\n"
        })
        .collect();

    let mut input = input.as_str();

    let arr = combinator::separated::<_, _, Vec<_>, _, (), _, _>(
        ..,
        token::take_while(
            ..,
            |ch: char| matches!(ch, '!' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'),
        ),
        (ascii::multispace0, ',', ascii::multispace0),
    );
    let mut arr = combinator::delimited('[', arr, ']');

    let arr = arr.parse_next(&mut input).expect("Failed to parse array");

    let int = ascii::dec_uint::<&str, usize, ()>;
    let len = combinator::preceded(ascii::multispace0, int)
        .parse_next(&mut input)
        .expect("Failed to parse len");
    // assert!(
    //     input.trim().is_empty(),
    //     "Got unexpected input after the end of the arguments to the macro"
    // );

    let arrow = combinator::delimited(ascii::multispace0::<_, ()>, "=>", ascii::multispace0);
    let callback = combinator::preceded(
        arrow,
        token::take_while(
            ..,
            |ch: char| matches!(ch, '!' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'),
        ),
    );
    // let callback = callback
    //     .parse_next(&mut input)
    let string = || {
        combinator::delimited(
            combinator::preceded(ascii::multispace0, '"'),
            token::take_while(.., |ch: char| ch != '"'),
            combinator::terminated('"', ascii::multispace0),
        )
    };

    let literal =
        |lit: &'static str| combinator::delimited(ascii::multispace0, lit, ascii::multispace0);
    let prefix = combinator::preceded(literal("prefix"), string());
    let suffix = combinator::preceded(literal("suffix"), string());
    let (prefix, suffix) = combinator::alt((
        callback.map(|cb: &str| (format!("{cb}("), ");".to_string())),
        (
            prefix.map(|pre: &str| pre.to_string()),
            suffix.map(|suf: &str| suf.to_string()),
        ),
    ))
    .parse_next(&mut input)
    .expect("Failed to parse callback");
    let separator = combinator::delimited(
        ascii::multispace0::<_, ()>,
        combinator::alt(("separated by", "separated", "separator", "sep")),
        ascii::multispace0,
    );
    let mut separator = combinator::preceded(separator, string());

    let separator = separator.parse_next(&mut input).unwrap_or("\n");

    let mut out = String::new();

    generate_combination_indices(arr.len(), len, |indices| {
        out += &prefix;
        indices
            .iter()
            .map(|&idx| arr[idx])
            .for_each(|val| out += val);
        indices.iter().map(|&idx| arr[idx]).for_each(|val| {
            out += ", ";
            out += val
        });
        out += &suffix;
        out += separator;
    });

    out.parse().unwrap()
}
