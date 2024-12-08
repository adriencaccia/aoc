use std::collections::HashMap;

use itertools::Itertools;

const GRID_SIZE: usize = 50;
const CHARS_LENGTH: usize = 26 + 26 + 10; // 26 lowercase, 26 uppercase, 10 digits
const CHAR_OCC: usize = 4; // each antenna has at most 4 occurrences

type CharPositions = HashMap<u8, Vec<(usize, usize)>>;

fn parse(input: &str) -> (CharPositions, usize) {
    let mut chars = CharPositions::with_capacity(CHARS_LENGTH);
    let mut size = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
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

pub fn part1(input: &str) -> u32 {
    let (chars, size) = parse(input);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter()
            .tuple_combinations()
            .for_each(|((a, b), (c, d))| unsafe {
                let i_diff = (*a as isize) - (*c as isize);
                let j_diff = (*b as isize) - (*d as isize);
                let i = (*a as isize) - 2 * i_diff;
                let j = (*b as isize) - 2 * j_diff;
                if i >= 0
                    && i < size as isize
                    && j >= 0
                    && j < size as isize
                    && !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize)
                {
                    new_antennas += 1;
                    a_nodes[i as usize][j as usize] = true;
                }
                let i = (*c as isize) + 2 * i_diff;
                let j = (*d as isize) + 2 * j_diff;
                if i >= 0
                    && i < size as isize
                    && j >= 0
                    && j < size as isize
                    && !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize)
                {
                    new_antennas += 1;
                    a_nodes[i as usize][j as usize] = true;
                }
            });

        sum + new_antennas
    })
}

pub fn part2(input: &str) -> u32 {
    let (chars, size) = parse(input);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter()
            .tuple_combinations()
            .for_each(|((a, b), (c, d))| unsafe {
                let i_diff = (*a as isize) - (*c as isize);
                let j_diff = (*b as isize) - (*d as isize);
                let mut i = *a as isize;
                let mut j = *b as isize;
                while i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                    if !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) {
                        new_antennas += 1;
                        a_nodes[i as usize][j as usize] = true;
                    }
                    i -= i_diff;
                    j -= j_diff;
                }
                let mut i = *c as isize;
                let mut j = *d as isize;
                while i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                    if !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) {
                        new_antennas += 1;
                        a_nodes[i as usize][j as usize] = true;
                    }
                    i += i_diff;
                    j += j_diff;
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
