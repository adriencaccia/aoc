const SIZE: usize = 140;

pub fn part1(input: &str) -> u32 {
    let mut grid = [['.'; SIZE]; SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    let len = grid.len();
    let mut sum = 0;

    for i in 0..len {
        for j in 0..len {
            match grid[i][j] {
                'X' => {
                    // XMAS >
                    if j + 3 < SIZE
                        && grid[i][j + 1] == 'M'
                        && grid[i][j + 2] == 'A'
                        && grid[i][j + 3] == 'S'
                    {
                        sum += 1;
                    }
                    // XMAS down
                    if i + 3 < SIZE
                        && grid[i + 1][j] == 'M'
                        && grid[i + 2][j] == 'A'
                        && grid[i + 3][j] == 'S'
                    {
                        sum += 1;
                    }
                    // XMAS diag right
                    if i + 3 < SIZE
                        && j + 3 < SIZE
                        && grid[i + 1][j + 1] == 'M'
                        && grid[i + 2][j + 2] == 'A'
                        && grid[i + 3][j + 3] == 'S'
                    {
                        sum += 1;
                    }
                    // XMAS diag left
                    if j > 2
                        && i + 3 < SIZE
                        && grid[i + 1][j - 1] == 'M'
                        && grid[i + 2][j - 2] == 'A'
                        && grid[i + 3][j - 3] == 'S'
                    {
                        sum += 1;
                    }
                }
                'S' => {
                    // SAMX >
                    if j + 3 < SIZE
                        && grid[i][j + 1] == 'A'
                        && grid[i][j + 2] == 'M'
                        && grid[i][j + 3] == 'X'
                    {
                        sum += 1;
                    }
                    // SAMX down
                    if i + 3 < SIZE
                        && grid[i + 1][j] == 'A'
                        && grid[i + 2][j] == 'M'
                        && grid[i + 3][j] == 'X'
                    {
                        sum += 1;
                    }
                    // SAMX diag right
                    if i + 3 < SIZE
                        && j + 3 < SIZE
                        && grid[i + 1][j + 1] == 'A'
                        && grid[i + 2][j + 2] == 'M'
                        && grid[i + 3][j + 3] == 'X'
                    {
                        sum += 1;
                    }
                    // SAMX diag left
                    if j > 2
                        && i + 3 < SIZE
                        && grid[i + 1][j - 1] == 'A'
                        && grid[i + 2][j - 2] == 'M'
                        && grid[i + 3][j - 3] == 'X'
                    {
                        sum += 1;
                    }
                }
                _ => {}
            }
        }
    }

    sum
}

fn is_x_mas(grid: &[[char; SIZE]; SIZE], i: usize, j: usize) -> bool {
    if i + 2 >= SIZE || j + 2 >= SIZE || grid[i + 1][j + 1] != 'A' {
        return false;
    }

    let tl = grid[i][j];
    let tr = grid[i][j + 2];
    let bl = grid[i + 2][j];
    let br = grid[i + 2][j + 2];

    // M.S
    // .A.
    // M.S
    if tl == 'M' && tr == 'S' && bl == 'M' && br == 'S' {
        return true;
    }
    // M.M
    // .A.
    // S.S
    if tl == 'M' && tr == 'M' && bl == 'S' && br == 'S' {
        return true;
    }
    // S.S
    // .A.
    // M.M
    if tl == 'S' && tr == 'S' && bl == 'M' && br == 'M' {
        return true;
    }
    // S.M
    // .A.
    // S.M
    if tl == 'S' && tr == 'M' && bl == 'S' && br == 'M' {
        return true;
    }
    false
}

pub fn part2(input: &str) -> u32 {
    let mut grid = [['.'; SIZE]; SIZE];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
        }
    }
    let len = grid.len();
    let mut sum = 0;

    for i in 0..len {
        for j in 0..len {
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
