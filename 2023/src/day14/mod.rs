use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use parse_display::{Display, FromStr};
use strum::{EnumIter, IntoEnumIterator};

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

#[derive(EnumIter, Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn tilt_grid(grid: &mut [Vec<Tile>], direction: Direction) {
    match direction {
        Direction::North => {
            for y in 0..grid[0].len() {
                let column = (0..grid.len()).map(|x| grid[x][y]).collect_vec();
                let tilted_column = tilt_rocks(column);
                for x in 0..grid.len() {
                    grid[x][y] = tilted_column[x];
                }
            }
        }
        Direction::West => {
            for (x, row) in grid.to_owned().iter().enumerate() {
                grid[x] = tilt_rocks(row.to_vec());
            }
        }
        Direction::South => {
            for y in 0..grid[0].len() {
                let mut column = (0..grid.len()).map(|x| grid[x][y]).collect_vec();
                column.reverse();
                let mut tilted_column = tilt_rocks(column);
                tilted_column.reverse();
                for x in 0..grid.len() {
                    grid[x][y] = tilted_column[x];
                }
            }
        }
        Direction::East => {
            for (x, row) in grid.to_owned().iter().enumerate() {
                let mut reversed_row = row.to_vec();
                reversed_row.reverse();
                let mut reversed_rocks = tilt_rocks(reversed_row);
                reversed_rocks.reverse();
                grid[x] = reversed_rocks;
            }
        }
    }
}

fn get_load(grid: &[Vec<Tile>]) -> u32 {
    grid.iter().rev().enumerate().fold(0, |acc, (idx, row)| {
        acc + row.iter().filter(|&&t| t == Tile::Round).count() * (idx + 1)
    }) as u32
}

fn do_cycle(grid: &mut [Vec<Tile>]) {
    for direction in Direction::iter().collect_vec() {
        tilt_grid(grid, direction);
    }
}

fn part2(mut grid: Vec<Vec<Tile>>) -> u32 {
    let mut grids: HashMap<String, (usize, u32)> = HashMap::new();
    let mut idx = 0;

    loop {
        do_cycle(&mut grid);
        idx += 1;

        let grid_str = grid.iter().map(|l| l.iter().join("")).join("\n");
        if let Some((cycle_start, _)) = grids.get(&grid_str) {
            let cycle_length = idx - cycle_start;
            let grid_idx = cycle_start + (1000000000 - cycle_start) % cycle_length;

            return grids
                .iter()
                .find_map(
                    |(_, &(idx, load))| {
                        if idx == grid_idx {
                            Some(load)
                        } else {
                            None
                        }
                    },
                )
                .unwrap();
        }

        grids.insert(grid_str, (idx, get_load(&grid)));
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

    let grid_p2 = grid.clone();

    tilt_grid(&mut grid, Direction::North);
    let part1 = get_load(&grid);
    let part2 = part2(grid_p2);

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
        assert_eq!(part2, 64);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 108918);
        assert_eq!(part2, 100310);
    }
}
