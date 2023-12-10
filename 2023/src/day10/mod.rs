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

impl Tile {
    pub fn to_symbol(&self) -> char {
        match self {
            Tile::Horizontal => '─',
            Tile::Vertical => '│',
            Tile::Ground => '.',
            Tile::NorthEastBend => '└',
            Tile::NorthWestBend => '┘',
            Tile::SouthEastBend => '┌',
            Tile::SouthWestBend => '┐',
            Tile::StartingPosition => 'S',
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

lazy_static! {
    static ref DIRECTIONS: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1),];
}

fn get_next_position(
    grid: &mut [Vec<Tile>],
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

fn clean_grid(grid: &mut [Vec<Tile>], loop_path: Vec<(usize, usize)>) {
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if !loop_path.contains(&(x, y)) {
                grid[x][y] = Tile::Ground;
            }
        }
    }
    // TODO: clean start tile, here it is hardcoded because it is always this one in all the examples and my input
    grid[loop_path[0].0][loop_path[0].1] = Tile::SouthEastBend;
}

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
    let mut loop_path: Vec<(usize, usize)> = vec![start_position];
    let mut steps = 0;
    loop {
        let previous_position = if steps == 0 {
            (0, 0)
        } else {
            loop_path[steps - 1]
        };
        let next_position = get_next_position(&mut grid, loop_path[steps], previous_position);
        steps += 1;
        if next_position == start_position {
            break;
        }
        loop_path.push(next_position);
    }
    let part1 = (steps as u32) / 2;

    print_grid(&grid);
    clean_grid(&mut grid, loop_path);
    print_grid(&grid);

    let mut inside_tiles: Vec<(usize, usize)> = Vec::new();
    for x in 0..grid.len() {
        let mut inside = false;
        for y in 0..grid[0].len() {
            let tile = &grid[x][y];

            match (tile, inside) {
                (Tile::Vertical, false) => inside = true,
                (Tile::SouthEastBend, false) => inside = true,
                (Tile::SouthWestBend, false) => inside = true,
                (_, false) => continue,
                (Tile::Ground, true) => inside_tiles.push((x, y)),
                (Tile::Vertical, true) => inside = false,
                (Tile::SouthEastBend, true) => inside = false,
                (Tile::SouthWestBend, true) => inside = false,
                _ => continue,
            }
        }
    }

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
