use std::io::BufRead;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

#[derive(Debug)]
struct State {
    races: Vec<Race>,
}

impl State {
    fn parse(lines: &[String]) -> Self {
        let time = lines[0]
            .strip_prefix("Time:")
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>();
        let distance = lines[1]
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

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let state = State::parse(&lines);

    println!(
        "part 1: {}",
        state
            .races
            .iter()
            .map(|race| find_root_distance(race.time, race.distance))
            .product::<i64>()
    );

    println!("part 2: {}", {
        let (time, distance) = state.races.iter().fold((0, 0), |(time, distance), race| {
            let time_log_10 = (race.time as f64).log10().ceil() as u32;
            let distance_log_10 = (race.distance as f64).log10().ceil() as u32;

            let time = time * 10i64.pow(time_log_10) + race.time;
            let distance = distance * 10i64.pow(distance_log_10) + race.distance;

            (time, distance)
        });

        find_root_distance(time, distance)
    });
}
