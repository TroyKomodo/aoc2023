use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Deck {
    cards: Vec<Card>,
    bid: i64,
}

#[derive(Debug)]
struct State {
    decks: Vec<Deck>,
}

impl State {
    fn parse(lines: &[String]) -> Self {
        Self {
            decks: lines
                .iter()
                .map(|line| {
                    let mut splits = line.split(' ');
                    let cards = splits
                        .next()
                        .unwrap()
                        .chars()
                        .map(|c| match c {
                            'A' => Card::A,
                            'K' => Card::K,
                            'Q' => Card::Q,
                            'J' => Card::J,
                            'T' => Card::T,
                            '9' => Card::N9,
                            '8' => Card::N8,
                            '7' => Card::N7,
                            '6' => Card::N6,
                            '5' => Card::N5,
                            '4' => Card::N4,
                            '3' => Card::N3,
                            '2' => Card::N2,
                            _ => unreachable!(),
                        })
                        .collect();

                    let bid = splits.next().unwrap().parse().unwrap();

                    Deck { cards, bid }
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DeckType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let state = State::parse(&lines);

    println!("part 1: {}", {
        let mut decks = state
            .decks
            .iter()
            .map(|deck| {
                // Determine the deck type
                let mut counts = [0; 14];
                for card in deck.cards.iter() {
                    counts[*card as usize] += 1;
                }

                let mut counted_counts = [0; 5];
                for count in counts.iter().copied() {
                    if count != 0 {
                        counted_counts[count as usize - 1] += 1;
                    }
                }

                let deck_type = match counted_counts {
                    [0, 0, 0, 0, 1] => DeckType::FiveOfAKind,
                    [1, 0, 0, 1, 0] => DeckType::FourOfAKind,
                    [0, 1, 1, 0, 0] => DeckType::FullHouse,
                    [2, 0, 1, 0, 0] => DeckType::ThreeOfAKind,
                    [1, 2, 0, 0, 0] => DeckType::TwoPair,
                    [3, 1, 0, 0, 0] => DeckType::OnePair,
                    [5, 0, 0, 0, 0] => DeckType::HighCard,
                    _ => unreachable!(),
                };

                (deck_type, deck)
            })
            .collect::<Vec<_>>();

        decks.sort_by(|(deck_type_1, deck_1), (deck_type_2, deck_2)| {
            if deck_type_1 != deck_type_2 {
                deck_type_2.cmp(deck_type_1)
            } else {
                deck_2.cards.cmp(&deck_1.cards)
            }
        });

        decks
            .iter()
            .enumerate()
            .map(|(idx, (_, deck))| deck.bid * (idx as i64 + 1))
            .sum::<i64>()
    });

    println!("part 2: {}", {
        let mut decks = state
            .decks
            .iter()
            .map(|deck| {
                // Determine the deck type
                let mut counts = [0; 14];
                let mut joker_count = 0;
                for card in deck.cards.iter().copied() {
                    if card == Card::J {
                        joker_count += 1;
                        continue;
                    }

                    counts[card as usize] += 1;
                }

                let mut counted_counts = [0; 5];
                for count in counts.iter().copied() {
                    if count != 0 {
                        counted_counts[count as usize - 1] += 1;
                    }
                }

                let deck_type = match (counted_counts, joker_count) {
                    ([0, 0, 0, 0, 0], 5)
                    | ([1, 0, 0, 0, 0], 4)
                    | ([0, 1, 0, 0, 0], 3)
                    | ([0, 0, 1, 0, 0], 2)
                    | ([0, 0, 0, 1, 0], 1)
                    | ([0, 0, 0, 0, 1], 0) => DeckType::FiveOfAKind,
                    ([1, 0, 0, 1, 0], 0)
                    | ([1, 0, 1, 0, 0], 1)
                    | ([1, 1, 0, 0, 0], 2)
                    | ([2, 0, 0, 0, 0], 3) => DeckType::FourOfAKind,
                    ([0, 1, 1, 0, 0], 0) | ([0, 2, 0, 0, 0], 1) => DeckType::FullHouse,
                    ([2, 0, 1, 0, 0], 0) | ([2, 1, 0, 0, 0], 1) | ([3, 0, 0, 0, 0], 2) => {
                        DeckType::ThreeOfAKind
                    }
                    ([1, 2, 0, 0, 0], 0) => DeckType::TwoPair,
                    ([3, 1, 0, 0, 0], 0) | ([4, 0, 0, 0, 0], 1) => DeckType::OnePair,
                    ([5, 0, 0, 0, 0], 0) => DeckType::HighCard,
                    _ => unreachable!(),
                };

                (deck_type, deck)
            })
            .collect::<Vec<_>>();

        decks.sort_by(|(deck_type_1, deck_1), (deck_type_2, deck_2)| {
            if deck_type_1 != deck_type_2 {
                return deck_type_2.cmp(deck_type_1);
            }

            for (card_1, card_2) in deck_1.cards.iter().zip(deck_2.cards.iter()) {
                if card_1 == card_2 {
                    continue;
                }

                return if *card_1 == Card::J {
                    std::cmp::Ordering::Less
                } else if *card_2 == Card::J {
                    std::cmp::Ordering::Greater
                } else {
                    card_2.cmp(card_1)
                };
            }

            unreachable!()
        });

        decks
            .iter()
            .enumerate()
            .map(|(idx, (_, deck))| deck.bid * (idx as i64 + 1))
            .sum::<i64>()
    });
}
