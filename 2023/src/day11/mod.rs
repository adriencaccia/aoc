use std::collections::HashSet;

use itertools::Itertools;

#[cfg(debug_assertions)]
fn print_grid(grid: &[Vec<char>]) {
    println!();
    for line in grid {
        println!("{}", line.iter().join(""));
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    print_grid(&grid);

    let mut inserted_x = 0;
    for x in 0..grid.len() {
        if grid[x + inserted_x].iter().all(|&c| c == '.') {
            grid.insert(x + inserted_x, grid[x + inserted_x].clone());
            inserted_x += 1;
        }
    }
    print_grid(&grid);

    let mut inserted_y = 0;
    for y in 0..grid[0].len() {
        let line = (0..grid.len())
            .map(|x| grid[x][y + inserted_y])
            .collect_vec();
        if line.iter().all(|&c| c == '.') {
            (0..grid.len()).for_each(|x| {
                grid[x].insert(y + inserted_y, '.');
            });
            inserted_y += 1;
        }
    }

    print_grid(&grid);

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    let mut part1 = galaxies
        .clone()
        .iter()
        .cartesian_product(galaxies.iter())
        .filter_map(|(&(x1, y1), &(x2, y2))| {
            if x1 == x2 && y1 == y2 {
                return None;
            }
            Some((x1.abs_diff(x2) + y1.abs_diff(y2)) as u32)
        })
        .sum();
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
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 374);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 9591768);
        assert_eq!(part2, 0);
    }
}
