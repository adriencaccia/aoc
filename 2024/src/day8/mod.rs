use std::collections::HashMap;

use itertools::Itertools;

const GRID_SIZE: usize = 50;
const CHARS_LENGTH: usize = 256;
const CHAR_OCC: usize = 20;

pub fn part1(input: &str) -> u32 {
    let mut grid = [[b' '; GRID_SIZE]; GRID_SIZE];
    let mut chars: HashMap<u8, Vec<(usize, usize)>> = HashMap::with_capacity(CHARS_LENGTH);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'.' {
                return;
            }
            if let Some(positions) = chars.get_mut(&c) {
                positions.push((i, j));
            } else {
                let mut positions = Vec::with_capacity(CHAR_OCC);
                positions.push((i, j));
                chars.insert(c, positions);
            }
        });
    });

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter()
            .cartesian_product(p.iter())
            .for_each(|((a, b), (c, d))| {
                if (a, b) == (c, d) {
                    return;
                }
                let i_diff = (*a as isize) - (*c as isize);
                let j_diff = (*b as isize) - (*d as isize);
                let a_1 = ((*a as isize) - 2 * i_diff, (*b as isize) - 2 * j_diff);
                let a_2 = ((*c as isize) + 2 * i_diff, (*d as isize) + 2 * j_diff);

                try_add_antenna(a_1, &mut grid, &mut new_antennas, &mut a_nodes);
                try_add_antenna(a_2, &mut grid, &mut new_antennas, &mut a_nodes);
            });

        sum + new_antennas
    })
}

fn try_add_antenna(
    antenna: (isize, isize),
    grid: &mut [[u8; 50]; 50],
    new_antennas: &mut u32,
    a_nodes: &mut [[bool; 50]; 50],
) {
    if antenna.0 >= 0
        && (antenna.0 as usize) < GRID_SIZE
        && antenna.1 >= 0
        && (antenna.1 as usize) < GRID_SIZE
    {
        let i = antenna.0 as usize;
        let j = antenna.1 as usize;
        match grid[i][j] {
            b'.' => {
                *new_antennas += 1;
                grid[i][j] = b'#';
                a_nodes[i][j] = true;
            }
            // empty check is just for example
            b' ' | b'#' => {}
            _ if !a_nodes[i][j] => {
                *new_antennas += 1;
                a_nodes[i][j] = true;
            }
            _ => {}
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let mut grid = [[b' '; GRID_SIZE]; GRID_SIZE];
    let mut chars: HashMap<u8, Vec<(usize, usize)>> = HashMap::with_capacity(CHARS_LENGTH);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'.' {
                return;
            }
            if let Some(positions) = chars.get_mut(&c) {
                positions.push((i, j));
            } else {
                let mut positions = Vec::with_capacity(CHAR_OCC);
                positions.push((i, j));
                chars.insert(c, positions);
            }
        });
    });

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter()
            .cartesian_product(p.iter())
            .for_each(|((a, b), (c, d))| {
                if (a, b) == (c, d) {
                    return;
                }
                let i_diff = (*a as isize) - (*c as isize);
                let j_diff = (*b as isize) - (*d as isize);
                for x in 0..30 {
                    let a_1 = ((*a as isize) - x * i_diff, (*b as isize) - x * j_diff);
                    let a_2 = ((*c as isize) + x * i_diff, (*d as isize) + x * j_diff);
                    try_add_antenna(a_1, &mut grid, &mut new_antennas, &mut a_nodes);
                    try_add_antenna(a_2, &mut grid, &mut new_antennas, &mut a_nodes);
                }
            });

        sum + new_antennas
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 359);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 34);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1293);
    }
}
