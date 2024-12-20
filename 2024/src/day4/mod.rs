const SIZE: usize = 140;

pub fn part1(input: &str) -> u16 {
    let mut grid = [[b'.'; SIZE]; SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        });
    });

    let mut sum = 0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            // Optimize pattern matching with direct byte comparisons
            if grid[i][j] == b'X' {
                // Horizontal XMAS
                if j + 3 < SIZE
                    && grid[i][j + 1] == b'M'
                    && grid[i][j + 2] == b'A'
                    && grid[i][j + 3] == b'S'
                {
                    sum += 1;
                }

                // Vertical XMAS
                if i + 3 < SIZE
                    && grid[i + 1][j] == b'M'
                    && grid[i + 2][j] == b'A'
                    && grid[i + 3][j] == b'S'
                {
                    sum += 1;
                }

                // Diagonal right XMAS
                if i + 3 < SIZE
                    && j + 3 < SIZE
                    && grid[i + 1][j + 1] == b'M'
                    && grid[i + 2][j + 2] == b'A'
                    && grid[i + 3][j + 3] == b'S'
                {
                    sum += 1;
                }

                // Diagonal left XMAS
                if i + 3 < SIZE
                    && j >= 3
                    && grid[i + 1][j - 1] == b'M'
                    && grid[i + 2][j - 2] == b'A'
                    && grid[i + 3][j - 3] == b'S'
                {
                    sum += 1;
                }
            }

            if grid[i][j] == b'S' {
                // Similar optimizations for SAMX patterns
                if j + 3 < SIZE
                    && grid[i][j + 1] == b'A'
                    && grid[i][j + 2] == b'M'
                    && grid[i][j + 3] == b'X'
                {
                    sum += 1;
                }

                if i + 3 < SIZE
                    && grid[i + 1][j] == b'A'
                    && grid[i + 2][j] == b'M'
                    && grid[i + 3][j] == b'X'
                {
                    sum += 1;
                }

                if i + 3 < SIZE
                    && j + 3 < SIZE
                    && grid[i + 1][j + 1] == b'A'
                    && grid[i + 2][j + 2] == b'M'
                    && grid[i + 3][j + 3] == b'X'
                {
                    sum += 1;
                }

                if i + 3 < SIZE
                    && j >= 3
                    && grid[i + 1][j - 1] == b'A'
                    && grid[i + 2][j - 2] == b'M'
                    && grid[i + 3][j - 3] == b'X'
                {
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[inline(always)]
fn is_x_mas(grid: &[[u8; SIZE]; SIZE], i: usize, j: usize) -> bool {
    if grid[i + 1][j + 1] != b'A' {
        return false;
    }

    let tl = grid[i][j];
    let tr = grid[i][j + 2];
    let bl = grid[i + 2][j];
    let br = grid[i + 2][j + 2];

    // M.S
    // .A.
    // M.S
    if tl == b'M' && tr == b'S' && bl == b'M' && br == b'S' {
        return true;
    }
    // M.M
    // .A.
    // S.S
    if tl == b'M' && tr == b'M' && bl == b'S' && br == b'S' {
        return true;
    }
    // S.S
    // .A.
    // M.M
    if tl == b'S' && tr == b'S' && bl == b'M' && br == b'M' {
        return true;
    }
    // S.M
    // .A.
    // S.M
    if tl == b'S' && tr == b'M' && bl == b'S' && br == b'M' {
        return true;
    }
    false
}

pub fn part2(input: &str) -> u16 {
    let mut grid = [[b'.'; SIZE]; SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        });
    });
    let mut sum = 0;

    for i in 0..SIZE - 2 {
        for j in 0..SIZE - 2 {
            if is_x_mas(&grid, i, j) {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
"};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT);

        assert_eq!(part1, 18);
    }

    #[test]
    fn test_part1() {
        let part1 = part1(include_str!("input.txt"));

        assert_eq!(part1, 2591);
    }

    #[test]
    fn test_example_part2() {
        let part2 = part2(EXAMPLE_INPUT);

        assert_eq!(part2, 9);
    }

    #[test]
    fn test_part2() {
        let part2 = part2(include_str!("input.txt"));

        assert_eq!(part2, 1880);
    }
}
