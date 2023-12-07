#[derive(Debug)]
enum Symbol {
    Number(u32),
    Star,
    Unknown(char),
    Dot,
}

#[derive(Debug)]
struct Node {
    symbol: Symbol,
    range: std::ops::Range<usize>,
}

#[derive(Debug)]
struct Row {
    nodes: Vec<Node>,
}

impl std::str::FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .enumerate()
            .try_fold(Self { nodes: Vec::new() }, |mut row, (idx, c)| {
                let symbol = match c {
                    '*' => Symbol::Star,
                    '.' => Symbol::Dot,
                    _ => {
                        if c.is_numeric() {
                            Symbol::Number(c.to_digit(10).unwrap())
                        } else {
                            Symbol::Unknown(c)
                        }
                    }
                };

                if let Some(node) = row.nodes.last_mut() {
                    match (&mut node.symbol, &symbol) {
                        (Symbol::Number(value), Symbol::Number(new_value)) => {
                            *value = *value * 10 + new_value;
                            node.range.end = idx + 1;
                        }
                        (Symbol::Unknown(a), Symbol::Unknown(b)) if a == b => {
                            node.range.end = idx + 1;
                        }
                        (Symbol::Dot, Symbol::Dot) => {
                            node.range.end = idx + 1;
                        }
                        (Symbol::Star, Symbol::Star) => {
                            node.range.end = idx + 1;
                        }
                        _ => {
                            row.nodes.push(Node {
                                symbol,
                                range: idx..idx + 1,
                            });
                        }
                    }
                } else {
                    row.nodes.push(Node {
                        symbol,
                        range: idx..idx + 1,
                    });
                }

                Ok(row)
            })
    }
}

#[derive(Debug, Default)]
struct State {
    rows: Vec<Row>,
}

impl<D: AsRef<str>> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        Self {
            rows: iter
                .into_iter()
                .map(|s| s.as_ref().parse())
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }
}

fn search_grid<R>(
    grid: &State,
    matcher: impl Fn(&Node) -> Option<R> + Clone + Copy,
    combiner: impl Fn(R, [&[Node]; 3]) -> Option<u32>,
) -> u32 {
    grid.rows
        .iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.nodes
                .iter()
                .filter_map(move |node| matcher(node).map(|value| (ridx, &node.range, value)))
        })
        .map(|(ridx, range, item)| {
            let rows = [
                ridx.checked_sub(1).and_then(|x| grid.rows.get(x)),
                grid.rows.get(ridx),
                grid.rows.get(ridx + 1),
            ];

            let mut iter = rows.into_iter().map(move |row| {
                let Some(row) = row else {
                    return &[] as &[Node];
                };

                let start = row
                    .nodes
                    .iter()
                    .position(|node| {
                        node.range.start <= range.start && node.range.end >= range.start
                    })
                    .unwrap();
                let end = row.nodes.len()
                    - row
                        .nodes
                        .iter()
                        .rev()
                        .position(|node| {
                            node.range.start <= range.end && node.range.end >= range.end
                        })
                        .unwrap();

                &row.nodes[start..end]
            });

            let rows = [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ];

            (item, rows)
        })
        .filter_map(|(item, rows)| combiner(item, rows))
        .sum::<u32>()
}

fn part1(state: &State) -> u32 {
    search_grid(
        state,
        |node| {
            if let Symbol::Number(value) = node.symbol {
                Some(value)
            } else {
                None
            }
        },
        |value, rows| {
            rows.into_iter()
                .flatten()
                .any(|node| matches!(node.symbol, Symbol::Star | Symbol::Unknown(_)))
                .then_some(value)
        },
    )
}

fn part2(state: &State) -> u32 {
    search_grid(
        state,
        |node| {
            if matches!(node.symbol, Symbol::Star) {
                Some(())
            } else {
                None
            }
        },
        |_, rows| {
            let mut iter = rows.into_iter().flatten().filter_map(|node| {
                if let Symbol::Number(value) = node.symbol {
                    Some(value)
                } else {
                    None
                }
            });

            let first = iter.next()?;
            let second = iter.next()?;

            Some(first * second)
        },
    )
}

fn main() {
    let state = aoc::get_input();

    println!("part 1: {}", part1(&state));

    println!("part 2: {}", part2(&state));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day3");
    const EXAMPLE_INPUT: &str = include_str!("../examples/day3");

    #[test]
    fn test_example_part_1() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part1(&state), 4361);
    }

    #[test]
    fn test_example_part_2() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part2(&state), 467835);
    }

    #[test]
    fn test_part_1() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part1(&state), 530849);
    }

    #[test]
    fn test_part_2() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part2(&state), 84900879);
    }
}
