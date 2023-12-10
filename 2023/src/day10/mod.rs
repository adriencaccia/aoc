use itertools::Itertools;
use lazy_static::lazy_static;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
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

lazy_static! {
    static ref DIRECTIONS: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1),];
}

fn get_next_position(
    grid: &[Vec<Tile>],
    current_position: (usize, usize),
    previous_position: (usize, usize),
) -> (usize, usize) {
    for (offset_x, offset_y) in DIRECTIONS.iter() {
        if current_position.0 == 0 && *offset_x == -1
            || current_position.0 == grid.len() - 1 && *offset_x == 1
            || current_position.1 == 0 && *offset_y == -1
            || current_position.1 == grid[0].len() - 1 && *offset_y == 1
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
        let new_tile = &grid[new_position.0][new_position.1];
        match (offset_x, offset_y, current_tile, new_tile) {
            // go above
            (-1, 0, Tile::StartingPosition, Tile::Vertical) => return new_position,
            (-1, 0, Tile::StartingPosition, Tile::SouthEastBend) => return new_position,
            (-1, 0, Tile::StartingPosition, Tile::SouthWestBend) => return new_position,
            (-1, 0, Tile::StartingPosition, Tile::StartingPosition) => return new_position,
            (-1, 0, Tile::Vertical, Tile::Vertical) => return new_position,
            (-1, 0, Tile::Vertical, Tile::SouthEastBend) => return new_position,
            (-1, 0, Tile::Vertical, Tile::SouthWestBend) => return new_position,
            (-1, 0, Tile::Vertical, Tile::StartingPosition) => return new_position,
            (-1, 0, Tile::NorthEastBend, Tile::Vertical) => return new_position,
            (-1, 0, Tile::NorthEastBend, Tile::SouthEastBend) => return new_position,
            (-1, 0, Tile::NorthEastBend, Tile::SouthWestBend) => return new_position,
            (-1, 0, Tile::NorthEastBend, Tile::StartingPosition) => return new_position,
            (-1, 0, Tile::NorthWestBend, Tile::Vertical) => return new_position,
            (-1, 0, Tile::NorthWestBend, Tile::SouthEastBend) => return new_position,
            (-1, 0, Tile::NorthWestBend, Tile::SouthWestBend) => return new_position,
            (-1, 0, Tile::NorthWestBend, Tile::StartingPosition) => return new_position,
            // go right
            (0, 1, Tile::StartingPosition, Tile::Horizontal) => return new_position,
            (0, 1, Tile::StartingPosition, Tile::NorthWestBend) => return new_position,
            (0, 1, Tile::StartingPosition, Tile::SouthWestBend) => return new_position,
            (0, 1, Tile::StartingPosition, Tile::StartingPosition) => return new_position,
            (0, 1, Tile::Horizontal, Tile::Horizontal) => return new_position,
            (0, 1, Tile::Horizontal, Tile::NorthWestBend) => return new_position,
            (0, 1, Tile::Horizontal, Tile::SouthWestBend) => return new_position,
            (0, 1, Tile::Horizontal, Tile::StartingPosition) => return new_position,
            (0, 1, Tile::NorthEastBend, Tile::Horizontal) => return new_position,
            (0, 1, Tile::NorthEastBend, Tile::NorthWestBend) => return new_position,
            (0, 1, Tile::NorthEastBend, Tile::SouthWestBend) => return new_position,
            (0, 1, Tile::NorthEastBend, Tile::StartingPosition) => return new_position,
            (0, 1, Tile::SouthEastBend, Tile::Horizontal) => return new_position,
            (0, 1, Tile::SouthEastBend, Tile::NorthWestBend) => return new_position,
            (0, 1, Tile::SouthEastBend, Tile::SouthWestBend) => return new_position,
            (0, 1, Tile::SouthEastBend, Tile::StartingPosition) => return new_position,
            // go below
            (1, 0, Tile::StartingPosition, Tile::Vertical) => return new_position,
            (1, 0, Tile::StartingPosition, Tile::NorthEastBend) => return new_position,
            (1, 0, Tile::StartingPosition, Tile::NorthWestBend) => return new_position,
            (1, 0, Tile::StartingPosition, Tile::StartingPosition) => return new_position,
            (1, 0, Tile::Vertical, Tile::Vertical) => return new_position,
            (1, 0, Tile::Vertical, Tile::NorthEastBend) => return new_position,
            (1, 0, Tile::Vertical, Tile::NorthWestBend) => return new_position,
            (1, 0, Tile::Vertical, Tile::StartingPosition) => return new_position,
            (1, 0, Tile::SouthEastBend, Tile::Vertical) => return new_position,
            (1, 0, Tile::SouthEastBend, Tile::NorthEastBend) => return new_position,
            (1, 0, Tile::SouthEastBend, Tile::NorthWestBend) => return new_position,
            (1, 0, Tile::SouthEastBend, Tile::StartingPosition) => return new_position,
            (1, 0, Tile::SouthWestBend, Tile::Vertical) => return new_position,
            (1, 0, Tile::SouthWestBend, Tile::NorthEastBend) => return new_position,
            (1, 0, Tile::SouthWestBend, Tile::NorthWestBend) => return new_position,
            (1, 0, Tile::SouthWestBend, Tile::StartingPosition) => return new_position,
            // go left
            (0, -1, Tile::StartingPosition, Tile::Horizontal) => return new_position,
            (0, -1, Tile::StartingPosition, Tile::NorthEastBend) => return new_position,
            (0, -1, Tile::StartingPosition, Tile::SouthEastBend) => return new_position,
            (0, -1, Tile::StartingPosition, Tile::StartingPosition) => return new_position,
            (0, -1, Tile::Horizontal, Tile::Horizontal) => return new_position,
            (0, -1, Tile::Horizontal, Tile::NorthEastBend) => return new_position,
            (0, -1, Tile::Horizontal, Tile::SouthEastBend) => return new_position,
            (0, -1, Tile::Horizontal, Tile::StartingPosition) => return new_position,
            (0, -1, Tile::NorthWestBend, Tile::Horizontal) => return new_position,
            (0, -1, Tile::NorthWestBend, Tile::NorthEastBend) => return new_position,
            (0, -1, Tile::NorthWestBend, Tile::SouthEastBend) => return new_position,
            (0, -1, Tile::NorthWestBend, Tile::StartingPosition) => return new_position,
            (0, -1, Tile::SouthWestBend, Tile::Horizontal) => return new_position,
            (0, -1, Tile::SouthWestBend, Tile::NorthEastBend) => return new_position,
            (0, -1, Tile::SouthWestBend, Tile::SouthEastBend) => return new_position,
            (0, -1, Tile::SouthWestBend, Tile::StartingPosition) => return new_position,
            _ => continue,
        }
    }

    unreachable!()
}

fn parse_input(input: &str) -> (u32, u32) {
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let start_position = find_starting_position(&grid);
    let mut part1 = 0;
    let mut previous_position = (0, 0);
    let mut current_position = start_position;
    loop {
        let next_position = get_next_position(&grid, current_position, previous_position);
        part1 += 1;
        if next_position == start_position {
            break;
        }
        previous_position = current_position;
        current_position = next_position;
    }
    part1 /= 2;

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
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 6754);
        assert_eq!(part2, 0);
    }
}
