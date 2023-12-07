use std::collections::HashSet;

#[derive(Debug)]
struct State {
    cards: Vec<Card>,
}

impl<D: AsRef<str>> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        Self {
            cards: iter
                .into_iter()
                .map(|s| s.as_ref().parse())
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Card {
    idx: usize,
    winning_numbers: HashSet<i32>,
    numbers: Vec<i32>,
}

impl std::str::FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card ").ok_or(())?;

        let mut splits = s.splitn(2, ':');

        let idx = splits
            .next()
            .ok_or(())?
            .trim()
            .parse::<usize>()
            .map_err(|_| ())?;
        let mut numbers = splits
            .next()
            .ok_or(())?
            .split('|')
            .map(|s| s.split(' ').filter_map(|s| s.trim().parse::<i32>().ok()));

        let winning_numbers = numbers.next().ok_or(())?.collect::<HashSet<i32>>();
        let numbers = numbers.next().ok_or(())?.collect::<Vec<i32>>();

        Ok(Self {
            idx,
            winning_numbers,
            numbers,
        })
    }
}

fn part_1(state: &State) -> i32 {
    state
        .cards
        .iter()
        .map(|card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count() as u32;
            if count == 0 {
                0
            } else {
                2_i32.pow(count - 1)
            }
        })
        .sum()
}

fn part_2(state: &State) -> i32 {
    state
        .cards
        .iter()
        .fold(vec![1; state.cards.len()], |mut acc, card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count();
            for i in card.idx..card.idx + count {
                acc[i] += acc[card.idx - 1];
            }

            acc
        })
        .iter()
        .sum()
}

fn main() {
    let state = aoc::get_input();

    println!("part 1: {}", part_1(&state));

    println!("part 2: {}", part_2(&state));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day4");
    const EXAMPLE_INPUT: &str = include_str!("../examples/day4");

    #[test]
    fn test_example_part_1() {
        let cards = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_1(&cards), 13);
    }

    #[test]
    fn test_example_part_2() {
        let cards = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_2(&cards), 30);
    }

    #[test]
    fn test_part_1() {
        let cards = aoc::get_input_from(INPUT);

        assert_eq!(part_1(&cards), 24733);
    }

    #[test]
    fn test_part_2() {
        let cards = aoc::get_input_from(INPUT);

        assert_eq!(part_2(&cards), 5422730);
    }
}
