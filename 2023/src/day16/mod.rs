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

impl Direction {
    pub fn to_usize(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::West => 1,
            Direction::South => 2,
            Direction::East => 3,
        }
    }
}

fn get_energized_tiles(grid: &Vec<Vec<Tile>>, beam: (isize, isize, Direction)) -> u32 {
    let mut energized_grid: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    // using a matrix to check visits is waayyyyy faster than using HashSet, as .contains will take more time as the HashSet grows
    let mut visited: Vec<Vec<[bool; 4]>> = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut beams: Vec<(isize, isize, Direction)> = vec![beam];

    while let Some((x, y, direction)) = beams.pop() {
        // skip beam if it has gone outside or is already visited
        if x < 0
            || grid.len() <= (x as usize)
            || y < 0
            || grid[0].len() <= (y as usize)
            || visited[x as usize][y as usize][direction.to_usize()]
        {
            continue;
        }
        visited[x as usize][y as usize][direction.to_usize()] = true;
        energized_grid[x as usize][y as usize] = true;
        let tile = grid[x as usize][y as usize];

        let new_beam = match (tile, direction) {
            (Tile::Empty | Tile::SplitH, Direction::East) => (x, y + 1, direction),
            (Tile::Empty | Tile::SplitV, Direction::South) => (x + 1, y, direction),
            (Tile::Empty | Tile::SplitH, Direction::West) => (x, y - 1, direction),
            (Tile::Empty | Tile::SplitV, Direction::North) => (x - 1, y, direction),
            //       \
            (Tile::MirrorNWSE, Direction::East) => (x + 1, y, Direction::South),
            (Tile::MirrorNWSE, Direction::South) => (x, y + 1, Direction::East),
            (Tile::MirrorNWSE, Direction::West) => (x - 1, y, Direction::North),
            (Tile::MirrorNWSE, Direction::North) => (x, y - 1, Direction::West),
            //       /
            (Tile::MirrorSWNE, Direction::East) => (x - 1, y, Direction::North),
            (Tile::MirrorSWNE, Direction::South) => (x, y - 1, Direction::West),
            (Tile::MirrorSWNE, Direction::West) => (x + 1, y, Direction::South),
            (Tile::MirrorSWNE, Direction::North) => (x, y + 1, Direction::East),
            (Tile::SplitH, _) => {
                beams.push((x, y + 1, Direction::East));
                (x, y - 1, Direction::West)
            }
            (Tile::SplitV, _) => {
                beams.push((x + 1, y, Direction::South));
                (x - 1, y, Direction::North)
            }
        };
        beams.push(new_beam);
    }

    energized_grid
        .into_iter()
        .map(|l| l.into_iter().map(|b| if b { 1 } else { 0 }).sum::<u32>())
        .sum()
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
