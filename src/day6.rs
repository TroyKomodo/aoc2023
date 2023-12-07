#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

#[derive(Debug)]
struct State {
    races: Vec<Race>,
}

impl<D: AsRef<str>> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        let mut iter = iter.into_iter();

        let time = iter
            .next()
            .unwrap()
            .as_ref()
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        let distance = iter
            .next()
            .unwrap()
            .as_ref()
            .strip_prefix("Distance:")
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();

        Self {
            races: time
                .into_iter()
                .zip(distance)
                .map(|(time, distance)| Race { time, distance })
                .collect::<Vec<_>>(),
        }
    }
}

fn find_root_distance(time: i64, distance: i64) -> i64 {
    let discriminant = (time * time) - 4 * distance;
    assert!(discriminant >= 0, "discriminant is negative");

    let sqrt = (discriminant as f64).sqrt();

    let root_1 = ((time as f64 - sqrt) / 2.0 + 1.0).floor() as i64;
    let root_2 = ((time as f64 + sqrt) / 2.0 - 1.0).ceil() as i64;

    root_2 - root_1 + 1
}

fn part_1(state: &State) -> i64 {
    state
        .races
        .iter()
        .map(|race| find_root_distance(race.time, race.distance))
        .product::<i64>()
}

fn part_2(state: &State) -> i64 {
    let (time, distance) = state.races.iter().fold((0, 0), |(time, distance), race| {
        let time_log_10 = (race.time as f64).log10().ceil() as u32;
        let distance_log_10 = (race.distance as f64).log10().ceil() as u32;

        let time = time * 10i64.pow(time_log_10) + race.time;
        let distance = distance * 10i64.pow(distance_log_10) + race.distance;

        (time, distance)
    });

    find_root_distance(time, distance)
}

fn main() {
    let state = aoc::get_input();

    println!("part 1: {}", part_1(&state));

    println!("part 2: {}", part_2(&state));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/day6");
    const EXAMPLE_INPUT: &str = include_str!("../examples/day6");

    #[test]
    fn test_example_part_1() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_1(&state), 288);
    }

    #[test]
    fn test_example_part_2() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_2(&state), 71503);
    }

    #[test]
    fn test_part_1() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_1(&state), 114400);
    }

    #[test]
    fn test_part_2() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_2(&state), 21039729);
    }
}
