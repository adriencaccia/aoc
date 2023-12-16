use std::collections::HashSet;

use itertools::Itertools;
use parse_display::{Display, FromStr};
use rayon::prelude::*;

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

fn get_energized_tiles(grid: &Vec<Vec<Tile>>, beam: (isize, isize, Direction)) -> u32 {
    let mut energized_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    let mut visited: HashSet<(isize, isize, Direction)> = HashSet::new();
    let mut beams: Vec<(isize, isize, Direction)> = vec![beam];

    loop {
        if beams.is_empty() {
            return energized_grid
                .into_iter()
                .map(|l| l.into_iter().map(|b| if b { 1 } else { 0 }).sum::<u32>())
                .sum();
        }
        let mut new_beams: Vec<(isize, isize, Direction)> = vec![];
        beams = beams
            .into_iter()
            .filter_map(|beam| {
                let (x, y, direction) = beam;
                // skip beams that have gone outside or are already visited
                if x < 0
                    || grid.len() <= (x as usize)
                    || y < 0
                    || grid[0].len() <= (y as usize)
                    || visited.contains(&(x, y, direction))
                {
                    return None;
                }
                visited.insert((x, y, direction));
                energized_grid[x as usize][y as usize] = true;
                let tile = grid[x as usize][y as usize];

                match (tile, direction) {
                    (Tile::Empty | Tile::SplitH, Direction::East) => Some((x, y + 1, direction)),
                    (Tile::Empty | Tile::SplitV, Direction::South) => Some((x + 1, y, direction)),
                    (Tile::Empty | Tile::SplitH, Direction::West) => Some((x, y - 1, direction)),
                    (Tile::Empty | Tile::SplitV, Direction::North) => Some((x - 1, y, direction)),
                    //       \
                    (Tile::MirrorNWSE, Direction::East) => Some((x + 1, y, Direction::South)),
                    (Tile::MirrorNWSE, Direction::South) => Some((x, y + 1, Direction::East)),
                    (Tile::MirrorNWSE, Direction::West) => Some((x - 1, y, Direction::North)),
                    (Tile::MirrorNWSE, Direction::North) => Some((x, y - 1, Direction::West)),
                    //       /
                    (Tile::MirrorSWNE, Direction::East) => Some((x - 1, y, Direction::North)),
                    (Tile::MirrorSWNE, Direction::South) => Some((x, y - 1, Direction::West)),
                    (Tile::MirrorSWNE, Direction::West) => Some((x + 1, y, Direction::South)),
                    (Tile::MirrorSWNE, Direction::North) => Some((x, y + 1, Direction::East)),
                    (Tile::SplitH, _) => {
                        new_beams.push((x, y + 1, Direction::East));
                        Some((x, y - 1, Direction::West))
                    }
                    (Tile::SplitV, _) => {
                        new_beams.push((x + 1, y, Direction::South));
                        Some((x - 1, y, Direction::North))
                    }
                }
            })
            .collect_vec();
        beams.extend(new_beams);
    }
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

    let part1 = get_energized_tiles(&grid, (0, 0, Direction::East));

    let rows = (0..grid.len())
        .into_par_iter()
        .map(|x| {
            get_energized_tiles(&grid, (x as isize, 0, Direction::East)).max(get_energized_tiles(
                &grid,
                (x as isize, grid[0].len() as isize, Direction::West),
            ))
        })
        .max()
        .unwrap();
    let columns = (0..grid[0].len())
        .into_par_iter()
        .map(|y| {
            get_energized_tiles(&grid, (0, y as isize, Direction::South)).max(get_energized_tiles(
                &grid,
                (grid.len() as isize, y as isize, Direction::North),
            ))
        })
        .max()
        .unwrap();

    let part2 = rows.max(columns);

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
        assert_eq!(part2, 51);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 7939);
        assert_eq!(part2, 8318);
    }
}
