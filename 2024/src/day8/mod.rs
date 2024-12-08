use std::collections::HashMap;

use itertools::Itertools;

const GRID_SIZE: usize = 50;
const CHARS_LENGTH: usize = 26 + 26 + 10;
const CHAR_OCC: usize = 4; // each antenna has at most 4 occurrences

type CharPositions = HashMap<u8, Vec<(usize, usize)>>;

fn parse(input: &str) -> (CharPositions, usize) {
    let mut chars = CharPositions::with_capacity(CHARS_LENGTH);
    let mut size = 0;
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(i, line)| {
            line.iter().enumerate().for_each(|(j, &c)| {
                if c != b'.' {
                    chars
                        .entry(c)
                        .or_insert_with(|| Vec::with_capacity(CHAR_OCC))
                        .push_within_capacity((i, j))
                        .unwrap();
                }
            });
            size += 1;
        });
    (chars, size)
}

type BitGrid = [u64; (GRID_SIZE * GRID_SIZE + 63) / 64];

#[inline(always)]
fn set_bit(grid: &mut BitGrid, i: usize, j: usize) -> bool {
    let idx = i * GRID_SIZE + j;
    let word_idx = idx >> 6;
    let bit_idx = idx & 63;
    let old = grid[word_idx];
    grid[word_idx] |= 1u64 << bit_idx;
    old & (1u64 << bit_idx) == 0
}

#[inline(always)]
fn process_antenna(
    size: usize,
    ai: isize,
    aj: isize,
    new_antennas: &mut u32,
    a_nodes: &mut BitGrid,
) {
    if ai >= 0
        && (ai as usize) < size
        && aj >= 0
        && (aj as usize) < size
        && set_bit(a_nodes, ai as usize, aj as usize)
    {
        *new_antennas += 1;
    }
}

pub fn part1(input: &str) -> u32 {
    let (chars, size) = parse(input);
    let mut a_nodes = [0u64; (GRID_SIZE * GRID_SIZE + 63) / 64];

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter().tuple_combinations().for_each(|((a, b), (c, d))| {
            let i_diff = (*a as isize) - (*c as isize);
            let j_diff = (*b as isize) - (*d as isize);
            let a_1 = ((*a as isize) - 2 * i_diff, (*b as isize) - 2 * j_diff);
            let a_2 = ((*c as isize) + 2 * i_diff, (*d as isize) + 2 * j_diff);

            process_antenna(size, a_1.0, a_1.1, &mut new_antennas, &mut a_nodes);
            process_antenna(size, a_2.0, a_2.1, &mut new_antennas, &mut a_nodes);
        });

        sum + new_antennas
    })
}

pub fn part2(input: &str) -> u32 {
    let (chars, size) = parse(input);
    let mut a_nodes = [0u64; (GRID_SIZE * GRID_SIZE + 63) / 64];

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter().tuple_combinations().for_each(|((a, b), (c, d))| {
            let i_diff = (*a as isize) - (*c as isize);
            let j_diff = (*b as isize) - (*d as isize);
            for x in 0..40 {
                let a_1 = ((*a as isize) - x * i_diff, (*b as isize) - x * j_diff);
                let a_2 = ((*c as isize) + x * i_diff, (*d as isize) + x * j_diff);

                process_antenna(size, a_1.0, a_1.1, &mut new_antennas, &mut a_nodes);
                process_antenna(size, a_2.0, a_2.1, &mut new_antennas, &mut a_nodes);
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
