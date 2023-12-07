#[derive(Debug)]
struct State {
    lines: Vec<String>,
}

impl<D: ToString> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        Self {
            lines: iter.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

fn parse_digits_helper(
    line: &str,
    mappings: &[(&str, i32)],
    range: impl Fn(usize) -> std::ops::Range<usize>,
    iter: impl Iterator<Item = (usize, char)>,
) -> Option<i32> {
    for (idx, c) in iter {
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

fn part1(state: &State) -> i32 {
    state
        .lines
        .iter()
        .filter_map(|line| parse_digits(line, &[]).map(|(first, last)| first * 10 + last))
        .sum()
}

fn part2(state: &State) -> i32 {
    state
        .lines
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
        .sum()
}

fn main() {
    let state = aoc::get_input();

    println!("part 1: {}", part1(&state));

    println!("part 2: {}", part2(&state));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day1");

    #[test]
    fn test_example_part_1() {
        const EXAMPLE_INPUT: &str = include_str!("../examples/day1_1");

        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part1(&state), 142);
    }

    #[test]
    fn test_example_part_2() {
        const EXAMPLE_INPUT: &str = include_str!("../examples/day1_2");

        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part2(&state), 281);
    }

    #[test]
    fn test_part_1() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part1(&state), 53080);
    }

    #[test]
    fn test_part_2() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part2(&state), 53268);
    }
}
