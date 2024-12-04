fn get(grid: &[Vec<char>], i: usize, j: usize) -> Option<&char> {
    grid.get(i).and_then(|line| line.get(j))
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let len = grid.len();
    let mut sum = 0;

    for i in 0..len {
        for j in 0..len {
            match get(&grid, i, j) {
                Some('X') => {
                    // XMAS >
                    if let Some('M') = get(&grid, i, j + 1) {
                        if let Some('A') = get(&grid, i, j + 2) {
                            if let Some('S') = get(&grid, i, j + 3) {
                                sum += 1;
                            }
                        }
                    }
                    // XMAS down
                    if let Some('M') = get(&grid, i + 1, j) {
                        if let Some('A') = get(&grid, i + 2, j) {
                            if let Some('S') = get(&grid, i + 3, j) {
                                sum += 1;
                            }
                        }
                    }
                    // XMAS diag right
                    if let Some('M') = get(&grid, i + 1, j + 1) {
                        if let Some('A') = get(&grid, i + 2, j + 2) {
                            if let Some('S') = get(&grid, i + 3, j + 3) {
                                sum += 1;
                            }
                        }
                    }
                    // XMAS diag left
                    if j > 2 {
                        if let Some('M') = get(&grid, i + 1, j - 1) {
                            if let Some('A') = get(&grid, i + 2, j - 2) {
                                if let Some('S') = get(&grid, i + 3, j - 3) {
                                    sum += 1;
                                }
                            }
                        }
                    }
                }
                Some('S') => {
                    // SAMX >
                    if let Some('A') = get(&grid, i, j + 1) {
                        if let Some('M') = get(&grid, i, j + 2) {
                            if let Some('X') = get(&grid, i, j + 3) {
                                sum += 1;
                            }
                        }
                    }
                    // SAMX down
                    if let Some('A') = get(&grid, i + 1, j) {
                        if let Some('M') = get(&grid, i + 2, j) {
                            if let Some('X') = get(&grid, i + 3, j) {
                                sum += 1;
                            }
                        }
                    }
                    // SAMX diag right
                    if let Some('A') = get(&grid, i + 1, j + 1) {
                        if let Some('M') = get(&grid, i + 2, j + 2) {
                            if let Some('X') = get(&grid, i + 3, j + 3) {
                                sum += 1;
                            }
                        }
                    }
                    // SAMX diag left
                    if j > 2 {
                        if let Some('A') = get(&grid, i + 1, j - 1) {
                            if let Some('M') = get(&grid, i + 2, j - 2) {
                                if let Some('X') = get(&grid, i + 3, j - 3) {
                                    sum += 1;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    sum
}

fn is_x_mas(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    if grid.get(i + 2).is_none() {
        return false;
    }
    if grid.get(i + 2).unwrap().get(j + 2).is_none() {
        return false;
    }
    let c = grid.get(i + 1).unwrap().get(j + 1).unwrap();
    if *c != 'A' {
        return false;
    }

    let tl = grid.get(i).unwrap().get(j).unwrap();
    let tr = grid.get(i).unwrap().get(j + 2).unwrap();
    let bl = grid.get(i + 2).unwrap().get(j).unwrap();
    let br = grid.get(i + 2).unwrap().get(j + 2).unwrap();

    // M.S
    // .A.
    // M.S
    if *tl == 'M' && *tr == 'S' && *bl == 'M' && *br == 'S' {
        return true;
    }
    // M.M
    // .A.
    // S.S
    if *tl == 'M' && *tr == 'M' && *bl == 'S' && *br == 'S' {
        return true;
    }
    // S.S
    // .A.
    // M.M
    if *tl == 'S' && *tr == 'S' && *bl == 'M' && *br == 'M' {
        return true;
    }
    // S.M
    // .A.
    // S.M
    if *tl == 'S' && *tr == 'M' && *bl == 'S' && *br == 'M' {
        return true;
    }
    false
}

pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
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
