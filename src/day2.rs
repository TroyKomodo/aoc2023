use std::io::BufRead;

struct Game {
    idx: i32,
    sets: Vec<Set>,
}

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

fn main() {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .collect::<Vec<String>>();

    println!(
        "part 1: {}",
        lines
            .iter()
            .filter_map(|line| {
                let game = line.parse::<Game>().ok()?;
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
            .sum::<i32>()
    );

    println!(
        "part 2: {}",
        lines
            .iter()
            .filter_map(|line| {
                let game = line.parse::<Game>().ok()?;
                let red = game.sets.iter().map(|set| set.red).max()?;
                let green = game.sets.iter().map(|set| set.green).max()?;
                let blue = game.sets.iter().map(|set| set.blue).max()?;

                Some(red * green * blue)
            })
            .sum::<i32>()
    );
}
