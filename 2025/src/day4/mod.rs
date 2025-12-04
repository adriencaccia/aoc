pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let mut rolls = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // skip if cell is a dot
            if grid[i][j] == b'.' {
                continue;
            }

            // all eight adjacent cells, discarding out-of-bounds
            let mut forklifts = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < grid.len() as isize
                        && nj >= 0
                        && nj < grid[i].len() as isize
                        && grid[ni as usize][nj as usize] == b'@'
                        && forklifts < 4
                    {
                        forklifts += 1;
                    }
                }
            }

            if forklifts < 4 {
                rolls += 1;
            }
        }
    }

    rolls
}

pub fn part2(input: &str) -> u32 {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let mut removed = 0;

    loop {
        let mut changed = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                // skip if cell is a dot
                if grid[i][j] == b'.' {
                    continue;
                }

                // all eight adjacent cells, discarding out-of-bounds
                let mut forklifts = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < grid.len() as isize
                            && nj >= 0
                            && nj < grid[i].len() as isize
                            && grid[ni as usize][nj as usize] == b'@'
                            && forklifts < 4
                        {
                            forklifts += 1;
                        }
                    }
                }

                if forklifts < 4 {
                    grid[i][j] = b'.';
                    changed = true;
                    removed += 1;
                }
            }
        }
        // // print the grid
        // for row in &grid {
        //     println!("{}", String::from_utf8_lossy(row));
        // }
        // println!();
        if !changed {
            break;
        }
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1480);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 8899);
    }
}
