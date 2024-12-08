use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(input: &str) -> (u32, usize) {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut empty_x: HashSet<usize> = HashSet::new();
    (0..grid.len()).for_each(|x| {
        if grid[x].iter().all(|&c| c == '.') {
            empty_x.insert(x);
        }
    });

    let mut empty_y: HashSet<usize> = HashSet::new();
    for y in 0..grid[0].len() {
        let line = (0..grid.len()).map(|x| grid[x][y]).collect_vec();
        if line.iter().all(|&c| c == '.') {
            empty_y.insert(y);
        }
    }

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    let replacement_count = 999999;

    let counts: (Vec<u32>, Vec<usize>) = galaxies
        .iter()
        .tuple_combinations()
        .filter_map(|(&(x1, y1), &(x2, y2))| {
            if x1 == x2 && y1 == y2 {
                return None;
            }
            let x_range = x1.min(x2)..x1.max(x2);
            let x_to_add = empty_x.iter().filter(|x| x_range.contains(x)).count();
            let y_range = y1.min(y2)..y1.max(y2);
            let y_to_add = empty_y.iter().filter(|y| y_range.contains(y)).count();

            Some((
                (x_range.len() + x_to_add + y_range.len() + y_to_add) as u32,
                (x_range.len()
                    + x_to_add * replacement_count
                    + y_range.len()
                    + y_to_add * replacement_count),
            ))
        })
        .unzip();
    let part1 = counts.0.iter().sum::<u32>();
    let part2 = counts.1.iter().sum::<usize>();
    (part1, part2)
}

pub fn main() -> (u32, usize) {
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
        assert_eq!(part2, 82000210);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 9591768);
        assert_eq!(part2, 746962097860);
    }
}
