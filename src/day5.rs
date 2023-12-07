use std::collections::HashMap;

#[derive(Debug, Default)]
struct State {
    seeds: Vec<i64>,
    conversions: Vec<Vec<(std::ops::Range<i64>, i64)>>,
}

impl<D: AsRef<str>> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        let lines = iter.into_iter().collect::<Vec<_>>();

        let empty_lines =
            std::iter::once(0).chain(lines.iter().enumerate().filter_map(|(idx, line)| {
                if line.as_ref().is_empty() {
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
                let header = lines.next().unwrap().as_ref();

                if let Some(seeds) = header.strip_prefix("seeds: ") {
                    state = seeds.split(' ').filter_map(|s| s.parse().ok()).collect();
                } else if let Some(conversions) = header.strip_suffix(" map:") {
                    let mut from_to = conversions.split('-');
                    let from = from_to.next().unwrap();
                    from_to.next().unwrap();
                    let to = from_to.next().unwrap();

                    let mut mappings = lines
                        .map(|line| {
                            let mut ranges = line
                                .as_ref()
                                .split(' ')
                                .filter_map(|s| s.parse::<i64>().ok());
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

const START: &str = "seed";
const END: &str = "location";

fn part_1(state: &State) -> i64 {
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
}

fn part_2(state: &State) -> i64 {
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
                                        Some(range_start + addition..mapped_range.end + addition)
                                    }
                                    (false, true, _, _) => {
                                        range.end = mapped_range.start;
                                        Some(mapped_range.start + addition..range_end + addition)
                                    }
                                    (false, false, true, true) => {
                                        range.end = mapped_range.start;
                                        old_seeds.push(mapped_range.end..range_end);
                                        Some(mapped_range.start + addition
                                            ..mapped_range.end + addition)
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
}

fn main() {
    let state = aoc::get_input();

    println!("part 1: {}", part_1(&state));

    println!("part 2: {}", part_2(&state));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day5");
    const EXAMPLE_INPUT: &str = include_str!("../examples/day5");

    #[test]
    fn test_example_part_1() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_1(&state), 35);
    }

    #[test]
    fn test_example_part_2() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_2(&state), 46);
    }

    #[test]
    fn test_part_1() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_1(&state), 174137457);
    }

    #[test]
    fn test_part_2() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_2(&state), 1493866);
    }
}
