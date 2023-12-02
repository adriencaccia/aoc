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

#[derive(FromStr, PartialEq, Debug)]
#[from_str(regex = r"((?P<red>\d+) red)|((?P<green>\d+) green)|((?P<blue>\d+) blue)")]
struct RevealedCubes {
    #[from_str(default)]
    red: u8,
    #[from_str(default)]
    green: u8,
    #[from_str(default)]
    blue: u8,
}

fn parse_input() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let game: Game = line.parse().unwrap();
            let is_valid = game
                .revealed_sets
                .split(';')
                .map(|set| {
                    let cubes = RevealedCubesNew::from_str(set).unwrap();

                    cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14
                })
                .all(|set| set);
            match is_valid {
                true => game.game_id,
                false => 0,
            }
        })
        .sum()
}

fn parse_input_2() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| -> u32 {
            let game: Game = line.parse().unwrap();
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            game.revealed_sets.split(';').for_each(|set| {
                let cubes = RevealedCubesNew::from_str(set).unwrap();

                if cubes.red > min_red {
                    min_red = cubes.red;
                }
                if cubes.green > min_green {
                    min_green = cubes.green;
                }
                if cubes.blue > min_blue {
                    min_blue = cubes.blue;
                }
            });
            min_red as u32 * min_green as u32 * min_blue as u32
        })
        .sum()
}

pub fn main() -> (u32, u32) {
    let part1 = parse_input();
    println!("part1 {}", part1);

    let part2 = parse_input_2();
    println!("part2 {}", part2);

    (part1, part2)
}

lazy_static! {
    static ref CUBES_REGEX: Regex =
        Regex::new(r"((?P<red>\d+) red)|((?P<green>\d+) green)|((?P<blue>\d+) blue)",).unwrap();
    static ref RED_REGEX: Regex = Regex::new(r"(?P<red>\d+) red",).unwrap();
    static ref GREEN_REGEX: Regex = Regex::new(r"(?P<green>\d+) green",).unwrap();
    static ref BLUE_REGEX: Regex = Regex::new(r"(?P<blue>\d+) blue",).unwrap();
}

#[derive(Debug, PartialEq)]
struct RevealedCubesNew {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct MyError {}

impl StdFromStr for RevealedCubesNew {
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

        // let captures = CUBES_REGEX.captures(string).unwrap();
        // let cap0 = captures.get(0);
        // println!("{:?}", cap0);
        // let red = captures.name("red");
        // println!("{:?}", red);
        // let blue = captures.name("blue");
        // println!("{:?}", blue);
        // Ok(Self {
        //     red: captures
        //         .name("red")
        //         .map_or(0, |value| value.as_str().parse().unwrap()),
        //     green: captures
        //         .name("green")
        //         .map_or(0, |value| value.as_str().parse().unwrap()),
        //     blue: captures
        //         .name("red")
        //         .map_or(0, |value| value.as_str().parse().unwrap()),
        // })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cubes() {
        assert_eq!(
            "3 blue, 4 red".parse(),
            Ok(RevealedCubes {
                red: 4,
                green: 0,
                blue: 3,
            })
        );
    }

    #[test]
    fn test_parse_cubes_new() {
        assert_eq!(
            RevealedCubesNew::from_str("3 blue, 4 red"),
            Ok(RevealedCubesNew {
                red: 4,
                green: 0,
                blue: 3,
            })
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(main().0, 2810);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(main().1, 53348);
    }
}
