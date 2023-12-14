use std::collections::VecDeque;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("O")]
    Round,
    #[display("#")]
    Square,
}

fn tilt_rocks(rocks: Vec<Tile>) -> Vec<Tile> {
    let mut available_spots: VecDeque<usize> = VecDeque::new();
    let mut tilted_rocks: Vec<Tile> = Vec::new();

    for (idx, rock) in rocks.iter().enumerate() {
        match rock {
            Tile::Empty => {
                available_spots.push_back(idx);
                tilted_rocks.push(*rock);
            }
            Tile::Square => {
                available_spots.clear();
                tilted_rocks.push(*rock);
            }
            Tile::Round => {
                if available_spots.is_empty() {
                    tilted_rocks.push(*rock);
                } else {
                    let swap_idx = available_spots.pop_front().unwrap();
                    available_spots.push_back(idx);
                    tilted_rocks[swap_idx] = *rock;
                    tilted_rocks.push(Tile::Empty);
                }
            }
        }
    }

    tilted_rocks
}

fn print_grid(grid: &[Vec<Tile>]) {
    for line in grid {
        println!("{}", line.iter().join(""));
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    for y in 0..grid[0].len() {
        let column = (0..grid.len()).map(|x| grid[x][y]).collect_vec();
        let tilted_column = tilt_rocks(column);
        for x in 0..grid.len() {
            grid[x][y] = tilted_column[x];
        }
    }

    print_grid(&grid);

    let part1 = grid.iter().rev().enumerate().fold(0, |acc, (idx, row)| {
        acc + row.iter().filter(|&&t| t == Tile::Round).count() * (idx + 1)
    }) as u32;
    let part2 = 0;
    (part1, part2)
}

pub fn main() -> (u32, u32) {
    let (part1, part2) = parse_input(include_str!("input.txt"));
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 136);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 108918);
        assert_eq!(part2, 0);
    }
}
