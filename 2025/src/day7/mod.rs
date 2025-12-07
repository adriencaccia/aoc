pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut splits = 0;

    for r in 1..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r - 1][c] == 'S' || grid[r - 1][c] == '|' {
                if grid[r][c] != '^' {
                    grid[r][c] = '|';
                } else {
                    splits += 1;
                    if c > 0 {
                        grid[r][c - 1] = '|';
                    }
                    if c + 1 < grid[r].len() {
                        grid[r][c + 1] = '|';
                    }
                }
            }
        }
    }

    splits
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<(char, usize)>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0)).collect())
        .collect();

    for r in 1..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r - 1][c].0 == 'S' || grid[r - 1][c].0 == '|' {
                if grid[r][c].0 != '^' {
                    grid[r][c].0 = '|';
                } else {
                    if c > 0 {
                        grid[r][c - 1].0 = '|';
                    }
                    if c + 1 < grid[r].len() {
                        grid[r][c + 1].0 = '|';
                    }
                }
            }
            if r == grid.len() - 1 && grid[r][c].0 == '|' {
                grid[r][c].1 = 1;
            }
        }
    }

    for r in (0..grid.len() - 1).rev() {
        for c in 0..grid[r].len() {
            if grid[r][c].0 == '|' {
                match grid[r + 1][c] {
                    ('|', n) => grid[r][c].1 = n,
                    ('^', _) => grid[r][c].1 = grid[r + 1][c - 1].1 + grid[r + 1][c + 1].1,
                    _ => panic!("Unexpected character"),
                }
            }
            if grid[r][c].0 == 'S' {
                return grid[r + 1][c].1;
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1642);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 47274292756692);
    }
}
