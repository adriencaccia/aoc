const SIZE: usize = 140;

pub fn part1(input: &str) -> u32 {
    let mut grid = [[b'.'; SIZE]; SIZE];

    // Use memcpy-like fast initialization
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| grid[i][j] = c);
    });

    // Parallel processing with Rayon
    (0..SIZE)
        .map(|i| {
            let mut local_sum = 0;
            for j in 0..SIZE {
                // XMAS horizontal pattern
                if grid[i][j] == b'X'
                    && j + 3 < SIZE
                    && grid[i][j + 1] == b'M'
                    && grid[i][j + 2] == b'A'
                    && grid[i][j + 3] == b'S'
                {
                    local_sum += 1;
                }

                // XMAS vertical pattern
                if grid[i][j] == b'X'
                    && i + 3 < SIZE
                    && grid[i + 1][j] == b'M'
                    && grid[i + 2][j] == b'A'
                    && grid[i + 3][j] == b'S'
                {
                    local_sum += 1;
                }

                // XMAS diagonal right pattern
                if grid[i][j] == b'X'
                    && i + 3 < SIZE
                    && j + 3 < SIZE
                    && grid[i + 1][j + 1] == b'M'
                    && grid[i + 2][j + 2] == b'A'
                    && grid[i + 3][j + 3] == b'S'
                {
                    local_sum += 1;
                }

                // XMAS diagonal left pattern
                if grid[i][j] == b'X'
                    && i + 3 < SIZE
                    && j >= 3
                    && grid[i + 1][j - 1] == b'M'
                    && grid[i + 2][j - 2] == b'A'
                    && grid[i + 3][j - 3] == b'S'
                {
                    local_sum += 1;
                }

                // SAMX horizontal pattern
                if grid[i][j] == b'S'
                    && j + 3 < SIZE
                    && grid[i][j + 1] == b'A'
                    && grid[i][j + 2] == b'M'
                    && grid[i][j + 3] == b'X'
                {
                    local_sum += 1;
                }

                // SAMX vertical pattern
                if grid[i][j] == b'S'
                    && i + 3 < SIZE
                    && grid[i + 1][j] == b'A'
                    && grid[i + 2][j] == b'M'
                    && grid[i + 3][j] == b'X'
                {
                    local_sum += 1;
                }

                // SAMX diagonal right pattern
                if grid[i][j] == b'S'
                    && i + 3 < SIZE
                    && j + 3 < SIZE
                    && grid[i + 1][j + 1] == b'A'
                    && grid[i + 2][j + 2] == b'M'
                    && grid[i + 3][j + 3] == b'X'
                {
                    local_sum += 1;
                }

                // SAMX diagonal left pattern
                if grid[i][j] == b'S'
                    && i + 3 < SIZE
                    && j >= 3
                    && grid[i + 1][j - 1] == b'A'
                    && grid[i + 2][j - 2] == b'M'
                    && grid[i + 3][j - 3] == b'X'
                {
                    local_sum += 1;
                }
            }
            local_sum
        })
        .sum()
}

fn is_x_mas(grid: &[[u8; SIZE]; SIZE], i: usize, j: usize) -> bool {
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

pub fn part2(input: &str) -> u32 {
    let mut grid = [[b'.'; SIZE]; SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
        });
    });

    (0..SIZE - 2)
        .map(|i| {
            let mut local_sum = 0;
            for j in 0..SIZE - 2 {
                if grid[i + 1][j + 1] != b'A' {
                    continue;
                }
                if is_x_mas(&grid, i, j) {
                    local_sum += 1;
                }
            }
            local_sum
        })
        .sum()
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
