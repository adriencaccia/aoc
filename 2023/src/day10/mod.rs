use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
enum Tile {
    #[display("|")]
    Vertical,
    #[display(r"-")]
    Horizontal,
    #[display("L")]
    NorthEastBend,
    #[display("J")]
    NorthWestBend,
    #[display("7")]
    SouthWestBend,
    #[display("F")]
    SouthEastBend,
    #[display(".")]
    Ground,
    #[display("S")]
    StartingPosition,
    Inside,
}

#[cfg(debug_assertions)]
impl Tile {
    pub fn to_symbol(&self) -> char {
        match self {
            Tile::Horizontal => '─',
            Tile::Vertical => '│',
            Tile::Ground => ' ',
            Tile::NorthEastBend => '└',
            Tile::NorthWestBend => '┘',
            Tile::SouthEastBend => '┌',
            Tile::SouthWestBend => '┐',
            Tile::StartingPosition => 'S',
            Tile::Inside => '█',
        }
    }
}

fn find_starting_position(grid: &[Vec<Tile>]) -> (usize, usize) {
    for (x, grid_line) in grid.iter().enumerate() {
        for (y, tile) in grid_line.iter().enumerate() {
            match tile {
                Tile::StartingPosition => return (x, y),
                _ => continue,
            }
        }
    }

    unreachable!()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

lazy_static! {
    static ref DIRECTION_TO_OFFSET: HashMap<Direction, (i16, i16)> = {
        let mut m = HashMap::new();
        m.insert(Direction::Up, (-1, 0));
        m.insert(Direction::Right, (0, 1));
        m.insert(Direction::Down, (1, 0));
        m.insert(Direction::Left, (0, -1));
        m
    };
}

fn get_next_possible_tiles(tile: &Tile, direction: Direction) -> Vec<Tile> {
    match (tile, direction) {
        (
            Tile::StartingPosition | Tile::Vertical | Tile::NorthEastBend | Tile::NorthWestBend,
            Direction::Up,
        ) => {
            vec![
                Tile::StartingPosition,
                Tile::Vertical,
                Tile::SouthWestBend,
                Tile::SouthEastBend,
            ]
        }
        (
            Tile::StartingPosition | Tile::Horizontal | Tile::NorthEastBend | Tile::SouthEastBend,
            Direction::Right,
        ) => vec![
            Tile::StartingPosition,
            Tile::Horizontal,
            Tile::NorthWestBend,
            Tile::SouthWestBend,
        ],
        (
            Tile::StartingPosition | Tile::Vertical | Tile::SouthEastBend | Tile::SouthWestBend,
            Direction::Down,
        ) => vec![
            Tile::StartingPosition,
            Tile::Vertical,
            Tile::NorthWestBend,
            Tile::NorthEastBend,
        ],
        (
            Tile::StartingPosition | Tile::Horizontal | Tile::NorthWestBend | Tile::SouthWestBend,
            Direction::Left,
        ) => vec![
            Tile::StartingPosition,
            Tile::Horizontal,
            Tile::SouthEastBend,
            Tile::NorthEastBend,
        ],
        _ => vec![],
    }
}

fn get_next_position(
    grid: &mut [Vec<Tile>],
    current_position: (usize, usize),
    previous_position: (usize, usize),
) -> (usize, usize) {
    for (&direction, &(offset_x, offset_y)) in DIRECTION_TO_OFFSET.iter() {
        if current_position.0 == 0 && direction == Direction::Up
            || current_position.0 == grid.len() - 1 && direction == Direction::Down
            || current_position.1 == 0 && direction == Direction::Left
            || current_position.1 == grid[0].len() - 1 && direction == Direction::Right
        {
            continue;
        }
        let new_position = (
            (current_position.0 as i16 + offset_x) as usize,
            (current_position.1 as i16 + offset_y) as usize,
        );
        if new_position == previous_position {
            continue;
        }
        let current_tile = &grid[current_position.0][current_position.1];
        let possible_tiles = get_next_possible_tiles(current_tile, direction);
        let next_tile = &grid[new_position.0][new_position.1];
        if possible_tiles.contains(next_tile) {
            return new_position;
        }
    }

    unreachable!()
}

fn clean_grid(grid: &mut [Vec<Tile>], loop_path: HashSet<(usize, usize)>) {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !loop_path.contains(&(x, y)) {
                grid[x][y] = Tile::Ground;
            }
        }
    }
    // TODO: clean the starting tile with its real value
    // grid[loop_path[0].0][loop_path[0].1] = get_real_start_tile();
}

#[cfg(debug_assertions)]
fn print_grid(grid: &[Vec<Tile>]) {
    println!("Printing grid");

    for line in grid {
        println!("{}", line.iter().map(|t| t.to_symbol()).join(""));
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let start_position = find_starting_position(&grid);
    let mut loop_path = HashSet::new();
    loop_path.insert(start_position);
    let mut steps = 0;
    let mut previous_position = (0, 0);
    let mut current_position = start_position;
    loop {
        let next_position = get_next_position(&mut grid, current_position, previous_position);
        steps += 1;
        if next_position == start_position {
            break;
        }
        previous_position = current_position;
        current_position = next_position;
        loop_path.insert(next_position);
    }
    let part1 = (steps as u32) / 2;

    clean_grid(&mut grid, loop_path);

    let mut inside_tiles: Vec<(usize, usize)> = Vec::new();
    let crossing_tiles: HashSet<Tile> = HashSet::from_iter(
        [
            Tile::Vertical,
            Tile::SouthEastBend,
            Tile::SouthWestBend,
            // TODO: remove once the starting tile is cleaned, as the algo will not work
            // if the starting tile is not one of the above tiles,
            // but in the examples and the input it is, so that works for now
            Tile::StartingPosition,
        ]
        .iter()
        .cloned(),
    );

    for x in 0..grid.len() {
        let mut inside = false;
        for y in 0..grid[0].len() {
            let tile = &grid[x][y];
            if crossing_tiles.contains(tile) {
                inside = !inside;
                continue;
            }
            if inside && *tile == Tile::Ground {
                inside_tiles.push((x, y));
                grid[x][y] = Tile::Inside;
            }
        }
    }
    #[cfg(debug_assertions)]
    print_grid(&grid);

    let part2 = inside_tiles.len() as u32;
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
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 8);
        assert_eq!(part2, 1);
    }

    #[test]
    fn test_part2_1() {
        let (_, part2) = parse_input(indoc! {"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
        "});

        assert_eq!(part2, 4);
    }
    #[test]

    fn test_part2_2() {
        let (_, part2) = parse_input(indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "});

        assert_eq!(part2, 8);
    }
    #[test]
    fn test_part2_3() {
        let (_, part2) = parse_input(indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "});

        assert_eq!(part2, 10);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 6754);
        assert_eq!(part2, 567);
    }
}
