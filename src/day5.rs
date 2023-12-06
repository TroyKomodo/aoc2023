use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Default)]
struct State {
    seeds: Vec<i64>,
    conversions: Vec<Vec<(std::ops::Range<i64>, i64)>>,
}

const START: &str = "seed";
const END: &str = "location";

impl State {
    fn from_lines(lines: &[String]) -> Self {
        let empty_lines =
            std::iter::once(0).chain(lines.iter().enumerate().filter_map(|(idx, line)| {
                if line.is_empty() {
                    Some(idx)
                } else {
                    None
                }
            }));

        let next_idx = empty_lines
            .clone()
            .skip(1)
            .chain(std::iter::once(lines.len()));

        let mut conversion_map = HashMap::new();

        let seeds = empty_lines
            .zip(next_idx)
            .map(|(idx, next_idx)| {
                if idx == 0 {
                    lines[idx..next_idx].iter()
                } else {
                    lines[idx + 1..next_idx].iter()
                }
            })
            .fold(Vec::new(), |mut state, mut lines| {
                let header = lines.next().unwrap();

                if let Some(seeds) = header.strip_prefix("seeds: ") {
                    state = seeds.split(' ').filter_map(|s| s.parse().ok()).collect();
                } else if let Some(conversions) = header.strip_suffix(" map:") {
                    let mut from_to = conversions.split('-');
                    let from = from_to.next().unwrap();
                    from_to.next().unwrap();
                    let to = from_to.next().unwrap();

                    let mut mappings = lines
                        .map(|line| {
                            let mut ranges = line.split(' ').filter_map(|s| s.parse::<i64>().ok());
                            let dest_start = ranges.next().unwrap();
                            let src_start = ranges.next().unwrap();
                            let length = ranges.next().unwrap();

                            (src_start..src_start + length, dest_start)
                        })
                        .collect::<Vec<_>>();

                    mappings.sort_by_key(|(src_start, _)| src_start.start);

                    conversion_map.insert(from, (to, mappings));
                } else {
                    panic!("invalid header: {}", header);
                }

                state
            });

        let mut conversions = Vec::with_capacity(conversion_map.len());

        let mut current = START;
        while current != END {
            let (next, mappings) = conversion_map.remove(current).unwrap();
            conversions.push(mappings);
            current = next;
        }

        Self { seeds, conversions }
    }
}

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let state = State::from_lines(&lines);

    println!(
        "part 1: {}",
        state
            .seeds
            .iter()
            .copied()
            .map(|seed| {
                state.conversions.iter().fold(seed, |seed, mappings| {
                    mappings
                        .binary_search_by(|(range, _)| {
                            if range.contains(&seed) {
                                std::cmp::Ordering::Equal
                            } else if range.start > seed {
                                std::cmp::Ordering::Greater
                            } else {
                                std::cmp::Ordering::Less
                            }
                        })
                        .ok()
                        .map(|idx| {
                            let (range, dest) = &mappings[idx];

                            dest + seed - range.start
                        })
                        .unwrap_or(seed)
                })
            })
            .min()
            .unwrap()
    );

    println!("part 2: {}", {
        state
            .conversions
            .iter()
            .fold(
                state
                    .seeds
                    .chunks(2)
                    .map(|seeds| seeds[0]..seeds[0] + seeds[1])
                    .collect::<Vec<_>>(),
                |mut seeds, mappings| {
                    let new_ranges = mappings
                        .iter()
                        .flat_map(|(mapped_range, dest)| {
                            let addition = dest - mapped_range.start;

                            let mut old_seeds = Vec::new();

                            let new_seeds = seeds
                                .iter_mut()
                                .filter_map(|range| {
                                    let range_start = range.start;
                                    let range_end = range.end;

                                    match (
                                        mapped_range.contains(&range.start),
                                        mapped_range.contains(&range.end),
                                        range.contains(&mapped_range.start),
                                        range.contains(&mapped_range.end),
                                    ) {
                                        (true, true, _, _) => {
                                            range.start = 0;
                                            range.end = 0;
                                            Some(range_start + addition..range_end + addition)
                                        }
                                        (true, false, _, _) => {
                                            range.start = mapped_range.end;
                                            Some(
                                                range_start + addition..mapped_range.end + addition,
                                            )
                                        }
                                        (false, true, _, _) => {
                                            range.end = mapped_range.start;
                                            Some(
                                                mapped_range.start + addition..range_end + addition,
                                            )
                                        }
                                        (false, false, true, true) => {
                                            range.end = mapped_range.start;
                                            old_seeds.push(mapped_range.end..range_end);
                                            Some(
                                                mapped_range.start + addition
                                                    ..mapped_range.end + addition,
                                            )
                                        }
                                        (false, false, false, false) => None,
                                        _ => unreachable!(
                                    "invalid range: range={range:?} mapped_range={mapped_range:?}"
                                ),
                                    }
                                })
                                .collect::<Vec<_>>();

                            seeds.retain(|r| r.start != r.end);
                            seeds.extend(old_seeds);

                            new_seeds
                        })
                        .collect::<Vec<_>>();

                    seeds.extend(new_ranges);

                    seeds
                },
            )
            .into_iter()
            .map(|r| r.start)
            .min()
            .unwrap()
    });
}
