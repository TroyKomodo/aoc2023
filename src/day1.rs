use std::io::BufRead;

fn parse_digits_helper(
    line: &str,
    mappings: &[(&str, i32)],
    range: impl Fn(usize) -> std::ops::Range<usize>,
    mut iter: impl Iterator<Item = (usize, char)>,
) -> Option<i32> {
    while let Some((idx, c)) = iter.next() {
        let r = range(idx);

        for mapping in mappings {
            if line[r.clone()].contains(mapping.0) {
                return Some(mapping.1);
            }
        }

        if c.is_numeric() {
            return Some(c.to_digit(10)? as i32);
        }
    }

    None
}

fn parse_digits(line: &str, mappings: &[(&str, i32)]) -> Option<(i32, i32)> {
    let first_digit =
        parse_digits_helper(line, mappings, |idx| 0..idx + 1, line.chars().enumerate())?;
    let last_digit = parse_digits_helper(
        line,
        mappings,
        |idx| line.len() - idx..line.len(),
        line.chars().rev().enumerate(),
    )?;

    Some((first_digit, last_digit))
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    println!(
        "part 1: {}",
        lines
            .iter()
            .filter_map(|line| { parse_digits(line, &[]).map(|(first, last)| first * 10 + last) })
            .sum::<i32>()
    );

    println!(
        "part 2: {}",
        lines
            .iter()
            .filter_map(|line| {
                parse_digits(
                    line,
                    &[
                        ("zero", 0),
                        ("one", 1),
                        ("two", 2),
                        ("three", 3),
                        ("four", 4),
                        ("five", 5),
                        ("six", 6),
                        ("seven", 7),
                        ("eight", 8),
                        ("nine", 9),
                    ],
                )
                .map(|(first, last)| first * 10 + last)
            })
            .sum::<i32>()
    );
}
