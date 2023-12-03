use std::io::BufRead;

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
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn parse_from_lines<'a, D: std::ops::Deref<Target = str> + 'a>(
        lines: impl Iterator<Item = &'a D>,
    ) -> Result<Self, ()> {
        Ok(Self {
            rows: lines
                .map(|line| line.parse::<Row>())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl std::str::FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| line.parse::<Row>())
            .collect::<Result<Vec<Row>, ()>>()
            .map(|rows| Self { rows })
    }
}

fn search_grid<R>(
    grid: &Grid,
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

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let grid = Grid::parse_from_lines(lines.iter()).unwrap();

    println!(
        "part 1: {}",
        search_grid(
            &grid,
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
            }
        ),
    );

    println!(
        "part 2: {}",
        search_grid(
            &grid,
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
            }
        ),
    );
}
