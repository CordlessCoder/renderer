use proc_macro::TokenStream;
use winnow::{ascii, combinator, prelude::*, stream::AsBStr, token};

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
    let range = combinator::alt((
        (int, "..=", int).map(|(start, _, end)| start..=end),
        (int, "..", int).map(|(start, _, end)| start..=end - 1),
        int.map(|val| val..=val),
    ));
    let range = combinator::preceded(ascii::multispace0, range)
        .parse_next(&mut input)
        .expect("Failed to parse range");
    // assert!(
    //     input.trim().is_empty(),
    //     "Got unexpected input after the end of the arguments to the macro"
    // );
    assert!(
        *range.end() <= arr.len(),
        "Got a range that would go beyond the provided array"
    );

    let arrow = combinator::delimited(ascii::multispace0::<_, ()>, "=>", ascii::multispace0);
    let mut callback = combinator::preceded(
        arrow,
        token::take_while(
            ..,
            |ch: char| matches!(ch, '!' | '_' | 'a'..='z' | 'A'..='Z' | '0'..='9'),
        ),
    );
    let callback = callback
        .parse_next(&mut input)
        .expect("Failed to parse callback");
    let separator = combinator::delimited(
        ascii::multispace0::<_, ()>,
        combinator::alt(("separated by", "separated", "separator", "sep")),
        ascii::multispace0,
    );
    let separator = combinator::terminated(separator, '"');
    let separator = combinator::preceded(separator, token::take_while(.., |ch: char| ch != '"'));
    let mut separator = combinator::terminated(separator, '"');

    let separator = separator.parse_next(&mut input).unwrap_or("\n");

    let mut out = String::new();
    for len in range.clone() {
        generate_combination_indices(arr.len(), len, |indices| {
            out += callback;
            out += "(";
            indices
                .iter()
                .map(|&idx| arr[idx])
                .for_each(|val| out += val);
            indices.iter().map(|&idx| arr[idx]).for_each(|val| {
                out += ", ";
                out += val
            });
            out += ");";
            out += separator;
        })
    }
    out.parse().unwrap()
}
