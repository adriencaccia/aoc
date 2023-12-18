use itertools::Itertools;
use parse_display::{Display, FromStr};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
#[display("{direction} {meters} ({color})")]
struct Instruction {
    direction: Direction,
    meters: usize,
    color: String,
}

#[derive(Display, FromStr, Eq, PartialEq, Debug, EnumIter)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

fn print_grid(grid: &[Vec<char>]) {
    for line in grid {
        println!("{}", line.iter().join(""));
    }
}

fn fill(grid: &mut Vec<Vec<char>>) {
    let size = grid.len();
    let start_inside = (size / 2 + 1, size / 2 + 1);
    let mut terrains: Vec<(usize, usize)> = vec![start_inside];

    while let Some((x, y)) = terrains.pop() {
        if grid[x][y] == '#' {
            continue;
        }
        grid[x][y] = '#';
        Direction::iter().for_each(|d| match d {
            Direction::Right => terrains.push((x, y + 1)),
            Direction::Down => terrains.push((x + 1, y)),
            Direction::Left => terrains.push((x, y - 1)),
            Direction::Up => terrains.push((x - 1, y)),
        });
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap());

    let size = 500;
    let mut grid = vec![vec!['.'; size * 2]; size];
    let mut cur_x = size / 2;
    let mut cur_y = size / 2;
    // let mut min_x = usize::MAX;
    // let mut first_inside = (usize::MAX, usize::MAX);
    grid[cur_x][cur_y] = '#';
    for ins in instructions {
        for _ in 0..ins.meters {
            match ins.direction {
                Direction::Right => cur_y += 1,
                Direction::Down => cur_x += 1,
                Direction::Left => cur_y -= 1,
                Direction::Up => cur_x -= 1,
            }

            grid[cur_x][cur_y] = '#';
            // if cur_x <= first_inside.0 && cur_y <= first_inside.1 {
            //     first_inside = (cur_x + 1, cur_y + 1)
            // }
        }
    }
    // print_grid(&grid);
    fill(&mut grid);

    let part1 = grid
        .iter()
        .fold(0, |acc, l| acc + l.iter().filter(|&c| *c == '#').count()) as u32;
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
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 62);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 106459);
        assert_eq!(part2, 0);
    }
}
