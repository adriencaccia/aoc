use std::collections::HashSet;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Tile {
    #[display(".")]
    Empty,
    #[display("/")]
    MirrorSWNE,
    #[display(r"\")]
    MirrorNWSE,
    #[display("-")]
    SplitH,
    #[display("|")]
    SplitV,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn parse_input(input: &str) -> (u32, u32) {
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut energized_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut beams: Vec<((isize, isize), Direction)> = vec![((0, 0), Direction::East)];
    let mut visited: HashSet<((isize, isize), Direction)> = HashSet::new();

    loop {
        if beams.iter().all(|&((x, y), direction)| {
            x < 0
                || grid.len() <= (x as usize)
                || y < 0
                || grid[0].len() <= (y as usize)
                || visited.contains(&((x, y), direction))
        }) {
            break;
        }
        let mut new_beams: Vec<((isize, isize), Direction)> = vec![];
        for beam_idx in 0..beams.len() {
            let ((x, y), direction) = beams[beam_idx];
            // skip beams that have gone outside or are already visited
            if x < 0
                || grid.len() <= (x as usize)
                || y < 0
                || grid[0].len() <= (y as usize)
                || visited.contains(&((x, y), direction))
            {
                continue;
            }
            visited.insert(((x, y), direction));
            energized_grid[x as usize][y as usize] = true;
            let tile = grid[x as usize][y as usize];

            match (tile, direction) {
                (Tile::Empty | Tile::SplitH, Direction::East) => {
                    beams[beam_idx] = ((x, y + 1), direction)
                }
                (Tile::Empty | Tile::SplitV, Direction::South) => {
                    beams[beam_idx] = ((x + 1, y), direction)
                }
                (Tile::Empty | Tile::SplitH, Direction::West) => {
                    beams[beam_idx] = ((x, y - 1), direction)
                }
                (Tile::Empty | Tile::SplitV, Direction::North) => {
                    beams[beam_idx] = ((x - 1, y), direction)
                }
                //       \
                (Tile::MirrorNWSE, Direction::East) => {
                    beams[beam_idx] = ((x + 1, y), Direction::South)
                }
                (Tile::MirrorNWSE, Direction::South) => {
                    beams[beam_idx] = ((x, y + 1), Direction::East)
                }
                (Tile::MirrorNWSE, Direction::West) => {
                    beams[beam_idx] = ((x - 1, y), Direction::North)
                }
                (Tile::MirrorNWSE, Direction::North) => {
                    beams[beam_idx] = ((x, y - 1), Direction::West)
                }
                //       /
                (Tile::MirrorSWNE, Direction::East) => {
                    beams[beam_idx] = ((x - 1, y), Direction::North)
                }
                (Tile::MirrorSWNE, Direction::South) => {
                    beams[beam_idx] = ((x, y - 1), Direction::West)
                }
                (Tile::MirrorSWNE, Direction::West) => {
                    beams[beam_idx] = ((x + 1, y), Direction::South)
                }
                (Tile::MirrorSWNE, Direction::North) => {
                    beams[beam_idx] = ((x, y + 1), Direction::East)
                }
                (Tile::SplitH, _) => {
                    beams[beam_idx] = ((x, y - 1), Direction::West);
                    new_beams.push(((x, y + 1), Direction::East));
                }
                (Tile::SplitV, _) => {
                    beams[beam_idx] = ((x - 1, y), Direction::North);
                    new_beams.push(((x + 1, y), Direction::South));
                }
            }
        }
        beams.extend(
            new_beams
                .clone()
                .into_iter()
                .filter(|b| !visited.contains(b)),
        );
    }

    let part1 = energized_grid
        .into_iter()
        .map(|l| l.into_iter().map(|b| if b { 1 } else { 0 }).sum::<u32>())
        .sum();
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

    const EXAMPLE_INPUT: &str = indoc! {r"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 46);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 7939);
        assert_eq!(part2, 0);
    }
}
