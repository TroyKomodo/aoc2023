use std::{collections::HashSet, io::BufRead};

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

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let cards = lines
        .iter()
        .filter_map(|line| line.parse::<Card>().ok())
        .collect::<Vec<Card>>();

    println!(
        "part 1: {}",
        cards
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
            .sum::<i32>()
    );

    println!("part 2: {}", {
        let mut card_counts = vec![1; cards.len()];

        cards.iter().for_each(|card| {
            let count = card
                .numbers
                .iter()
                .filter(|n| card.winning_numbers.contains(n))
                .count();
            for i in card.idx..card.idx + count {
                card_counts[i] += card_counts[card.idx - 1];
            }
        });

        card_counts.iter().sum::<i32>()
    });
}
