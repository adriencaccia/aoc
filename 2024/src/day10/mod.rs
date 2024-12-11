use std::collections::VecDeque;

const SIZE: usize = 53;
const ZEROS_SIZE: usize = 1000;

fn parse(input: &str) -> ([[u8; SIZE]; SIZE], Vec<(usize, usize)>) {
    let mut grid = [[b'.'; SIZE]; SIZE];
    let mut zeros = Vec::with_capacity(ZEROS_SIZE);
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'0' {
                zeros.push((i, j));
            }
        });
    });

    (grid, zeros)
}

fn find_trailhead_score(grid: &[[u8; SIZE]; SIZE], (i, j): (usize, usize)) -> u16 {
    let mut visited = [[false; SIZE]; SIZE];
    let mut stack = Vec::with_capacity(SIZE * SIZE);
    let mut score = 0;
    stack.push((i, j));
    while let Some((i, j)) = stack.pop() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        let elevation = grid[i][j];
        // only for example input
        if elevation == b'.' {
            continue;
        }
        if elevation == b'9' {
            score += 1;
        }

        if i > 0 && grid[i - 1][j] == elevation + 1 {
            stack.push((i - 1, j));
        }
        if i < SIZE - 1 && grid[i + 1][j] == elevation + 1 {
            stack.push((i + 1, j));
        }
        if j > 0 && grid[i][j - 1] == elevation + 1 {
            stack.push((i, j - 1));
        }
        if j < SIZE - 1 && grid[i][j + 1] == elevation + 1 {
            stack.push((i, j + 1));
        }
    }

    score
}

pub fn part1(input: &str) -> u16 {
    let (grid, zeros) = parse(input);

    zeros
        .iter()
        .fold(0, |sum, &zero| sum + find_trailhead_score(&grid, zero))
}

fn find_trailhead_rating(grid: &[[u8; SIZE]; SIZE], (i, j): (usize, usize)) -> u16 {
    let mut stack = VecDeque::with_capacity(SIZE * SIZE);
    let mut visits = [[0; SIZE]; SIZE];
    let mut rating = 0;
    stack.push_back((i, j));
    visits[i][j] = 1;
    while let Some((i, j)) = stack.pop_front() {
        let elevation = grid[i][j];
        // only for example input
        if elevation == b'.' {
            continue;
        }
        if elevation == b'9' {
            rating += visits[i][j];
            continue;
        }
        if i > 0 && grid[i - 1][j] == elevation + 1 {
            if visits[i - 1][j] == 0 {
                stack.push_back((i - 1, j));
            }
            visits[i - 1][j] += visits[i][j];
        }
        if i < SIZE - 1 && grid[i + 1][j] == elevation + 1 {
            if visits[i + 1][j] == 0 {
                stack.push_back((i + 1, j));
            }
            visits[i + 1][j] += visits[i][j];
        }
        if j > 0 && grid[i][j - 1] == elevation + 1 {
            if visits[i][j - 1] == 0 {
                stack.push_back((i, j - 1));
            }
            visits[i][j - 1] += visits[i][j];
        }
        if j < SIZE - 1 && grid[i][j + 1] == elevation + 1 {
            if visits[i][j + 1] == 0 {
                stack.push_back((i, j + 1));
            }
            visits[i][j + 1] += visits[i][j];
        }
    }

    rating
}

pub fn part2(input: &str) -> u16 {
    let (grid, zeros) = parse(input);

    zeros
        .iter()
        .fold(0, |sum, &zero| sum + find_trailhead_rating(&grid, zero))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 36);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 776);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 81);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1657);
    }
}
