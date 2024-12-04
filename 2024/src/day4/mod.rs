const SIZE: usize = 140;

pub fn part1(input: &str) -> u16 {
    // Use unsafe for zero-initialization and potential performance gains
    let mut grid = unsafe { std::mem::zeroed::<[[u8; SIZE]; SIZE]>() };

    // Faster grid population using direct memory access
    unsafe {
        let mut ptr = grid.as_mut_ptr() as *mut u8;
        for line in input.lines() {
            std::ptr::copy_nonoverlapping(line.as_bytes().as_ptr(), ptr, line.len());
            ptr = ptr.add(SIZE);
        }
    }

    let mut sum = 0;

    // Adjusted bounds to prevent overflow
    for i in 0..SIZE {
        for j in 0..SIZE {
            // Horizontal XMAS (right)
            sum += (j < SIZE - 3
                && grid[i][j] == b'X'
                && grid[i][j + 1] == b'M'
                && grid[i][j + 2] == b'A'
                && grid[i][j + 3] == b'S') as u16;

            // Vertical XMAS (down)
            sum += (i < SIZE - 3
                && grid[i][j] == b'X'
                && grid[i + 1][j] == b'M'
                && grid[i + 2][j] == b'A'
                && grid[i + 3][j] == b'S') as u16;

            // Diagonal right XMAS
            sum += (i < SIZE - 3
                && j < SIZE - 3
                && grid[i][j] == b'X'
                && grid[i + 1][j + 1] == b'M'
                && grid[i + 2][j + 2] == b'A'
                && grid[i + 3][j + 3] == b'S') as u16;

            // Diagonal left XMAS (with bounds check)
            sum += (i < SIZE - 3
                && j >= 3
                && grid[i][j] == b'X'
                && grid[i + 1][j - 1] == b'M'
                && grid[i + 2][j - 2] == b'A'
                && grid[i + 3][j - 3] == b'S') as u16;

            // Horizontal SAMX (right)
            sum += (j < SIZE - 3
                && grid[i][j] == b'S'
                && grid[i][j + 1] == b'A'
                && grid[i][j + 2] == b'M'
                && grid[i][j + 3] == b'X') as u16;

            // Vertical SAMX (down)
            sum += (i < SIZE - 3
                && grid[i][j] == b'S'
                && grid[i + 1][j] == b'A'
                && grid[i + 2][j] == b'M'
                && grid[i + 3][j] == b'X') as u16;

            // Diagonal right SAMX
            sum += (i < SIZE - 3
                && j < SIZE - 3
                && grid[i][j] == b'S'
                && grid[i + 1][j + 1] == b'A'
                && grid[i + 2][j + 2] == b'M'
                && grid[i + 3][j + 3] == b'X') as u16;

            // Diagonal left SAMX (with bounds check)
            sum += (i < SIZE - 3
                && j >= 3
                && grid[i][j] == b'S'
                && grid[i + 1][j - 1] == b'A'
                && grid[i + 2][j - 2] == b'M'
                && grid[i + 3][j - 3] == b'X') as u16;
        }
    }

    sum
}

#[inline(always)]
fn is_x_mas(grid: &[[u8; SIZE]; SIZE], i: usize, j: usize) -> bool {
    // Early quick exit
    grid[i + 1][j + 1] == b'A'
        && ((grid[i][j] == b'M'
            && grid[i][j + 2] == b'S'
            && grid[i + 2][j] == b'M'
            && grid[i + 2][j + 2] == b'S')
            || (grid[i][j] == b'M'
                && grid[i][j + 2] == b'M'
                && grid[i + 2][j] == b'S'
                && grid[i + 2][j + 2] == b'S')
            || (grid[i][j] == b'S'
                && grid[i][j + 2] == b'S'
                && grid[i + 2][j] == b'M'
                && grid[i + 2][j + 2] == b'M')
            || (grid[i][j] == b'S'
                && grid[i][j + 2] == b'M'
                && grid[i + 2][j] == b'S'
                && grid[i + 2][j + 2] == b'M'))
}

pub fn part2(input: &str) -> u16 {
    // Use same unsafe zero-initialization technique
    let mut grid = unsafe { std::mem::zeroed::<[[u8; SIZE]; SIZE]>() };

    // Faster grid population using direct memory access
    unsafe {
        let mut ptr = grid.as_mut_ptr() as *mut u8;
        for line in input.lines() {
            std::ptr::copy_nonoverlapping(line.as_bytes().as_ptr(), ptr, line.len());
            ptr = ptr.add(SIZE);
        }
    }

    let mut sum = 0;

    for i in 0..SIZE - 2 {
        for j in 0..SIZE - 2 {
            sum += is_x_mas(&grid, i, j) as u16;
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
