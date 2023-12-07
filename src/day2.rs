#[derive(Debug)]
struct State {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    idx: i32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

impl std::str::FromStr for Set {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let colors = s.split(',');
        for color in colors {
            let mut color = color.trim().splitn(2, ' ');
            let count = color.next().ok_or(())?.parse::<i32>().map_err(|_| ())?;
            let color = color.next().ok_or(())?.trim();
            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => return Err(()),
            }
        }

        Ok(Set { red, green, blue })
    }
}

impl std::str::FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(())?;
        let mut splits = s.splitn(2, ':');
        let idx = splits
            .next()
            .ok_or(())?
            .trim()
            .parse::<i32>()
            .map_err(|_| ())?;
        let sets = splits.next().ok_or(())?.trim().split(';');
        let sets = sets
            .map(std::str::FromStr::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Game { idx, sets })
    }
}

impl<D: AsRef<str>> std::iter::FromIterator<D> for State {
    fn from_iter<T: IntoIterator<Item = D>>(iter: T) -> Self {
        Self {
            games: iter
                .into_iter()
                .map(|s| s.as_ref().parse())
                .collect::<Result<_, _>>()
                .unwrap(),
        }
    }
}

fn part_1(state: &State) -> i32 {
    state
        .games
        .iter()
        .filter_map(|game| {
            if game
                .sets
                .iter()
                .any(|set| set.red > 12 || set.green > 13 || set.blue > 14)
            {
                None
            } else {
                Some(game.idx)
            }
        })
        .sum()
}

fn part_2(state: &State) -> i32 {
    state
        .games
        .iter()
        .filter_map(|game| {
            let red = game.sets.iter().map(|set| set.red).max()?;
            let green = game.sets.iter().map(|set| set.green).max()?;
            let blue = game.sets.iter().map(|set| set.blue).max()?;

            Some(red * green * blue)
        })
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

    const INPUT: &str = include_str!("../inputs/day2");
    const EXAMPLE_INPUT: &str = include_str!("../examples/day2");

    #[test]
    fn test_example_part_1() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_1(&state), 8);
    }

    #[test]
    fn test_example_part_2() {
        let state = aoc::get_input_from(EXAMPLE_INPUT);

        assert_eq!(part_2(&state), 2286);
    }

    #[test]
    fn test_part_1() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_1(&state), 2486);
    }

    #[test]
    fn test_part_2() {
        let state = aoc::get_input_from(INPUT);

        assert_eq!(part_2(&state), 87984);
    }
}
