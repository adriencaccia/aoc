use itertools::MultiUnzip;
use lazy_static::lazy_static;
use parse_display::FromStr;
use regex::Regex;
use std::{str::FromStr as StdFromStr, u32};

// only 12 red cubes, 13 green cubes, and 14 blue cubes

#[derive(FromStr, PartialEq, Debug)]
#[display("Game {game_id}: {revealed_sets}")]
struct Game {
    game_id: u32,
    revealed_sets: String,
}

fn parse_input(input: &str) -> (u32, u32) {
    let (part1, part2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let game: Game = line.parse().unwrap();
            let cubes_sets = game
                .revealed_sets
                .split(';')
                .map(|set| RevealedCubes::from_str(set).unwrap());

            let is_game_valid = cubes_sets
                .clone()
                .map(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
                .all(|set| set);

            let (red, green, blue): (Vec<u32>, Vec<u32>, Vec<u32>) = cubes_sets
                .map(|cubes| (cubes.red as u32, cubes.green as u32, cubes.blue as u32))
                .multiunzip();

            (
                match is_game_valid {
                    true => game.game_id,
                    false => 0,
                },
                red.into_iter().max().unwrap()
                    * green.into_iter().max().unwrap()
                    * blue.into_iter().max().unwrap(),
            )
        })
        .unzip();

    (part1.into_iter().sum(), part2.into_iter().sum())
}

pub fn main() -> (u32, u32) {
    let (part1, part2) = parse_input(include_str!("input.txt"));
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

lazy_static! {
    static ref RED_REGEX: Regex = Regex::new(r"(?P<red>\d+) red",).unwrap();
    static ref GREEN_REGEX: Regex = Regex::new(r"(?P<green>\d+) green",).unwrap();
    static ref BLUE_REGEX: Regex = Regex::new(r"(?P<blue>\d+) blue",).unwrap();
}

#[derive(Debug, PartialEq)]
struct RevealedCubes {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct MyError {}

impl StdFromStr for RevealedCubes {
    type Err = MyError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let red = RED_REGEX.captures(string).map_or(0, |cap| {
            cap.name("red")
                .map_or(0, |value| value.as_str().parse().unwrap())
        });
        let green = GREEN_REGEX.captures(string).map_or(0, |cap| {
            cap.name("green")
                .map_or(0, |value| value.as_str().parse().unwrap())
        });
        let blue = BLUE_REGEX.captures(string).map_or(0, |cap| {
            cap.name("blue")
                .map_or(0, |value| value.as_str().parse().unwrap())
        });

        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cubes_new() {
        assert_eq!(
            RevealedCubes::from_str("3 blue, 4 red"),
            Ok(RevealedCubes {
                red: 4,
                green: 0,
                blue: 3,
            })
        );
    }

    const EXAMPLE_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT.trim());

        assert_eq!(part1, 8);
        assert_eq!(part2, 2286);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 2810);
        assert_eq!(part2, 69110);
    }
}
