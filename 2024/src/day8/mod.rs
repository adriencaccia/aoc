use std::collections::HashMap;

use itertools::Itertools;

const GRID_SIZE: usize = 50;
const CHARS_LENGTH: usize = 53;
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
        p.iter().tuple_combinations().for_each(|((a, b), (c, d))| {
            let i_diff = (*a as isize) - (*c as isize);
            let j_diff = (*b as isize) - (*d as isize);
            let a_1 = ((*a as isize) - 2 * i_diff, (*b as isize) - 2 * j_diff);
            let a_2 = ((*c as isize) + 2 * i_diff, (*d as isize) + 2 * j_diff);

            try_add_antenna(a_1, &mut new_antennas, &mut a_nodes, size);
            try_add_antenna(a_2, &mut new_antennas, &mut a_nodes, size);
        });

        sum + new_antennas
    })
}

#[inline(always)]
fn try_add_antenna(
    antenna: (isize, isize),
    new_antennas: &mut u32,
    a_nodes: &mut [[bool; 50]; 50],
    size: usize,
) {
    if antenna.0 >= 0
        && (antenna.0 as usize) < size
        && antenna.1 >= 0
        && (antenna.1 as usize) < size
        && !a_nodes[antenna.0 as usize][antenna.1 as usize]
    {
        *new_antennas += 1;
        a_nodes[antenna.0 as usize][antenna.1 as usize] = true;
    }
}

pub fn part2(input: &str) -> u32 {
    let (chars, size) = parse(input);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];

    chars.into_iter().fold(0, |sum, (_, p)| {
        let mut new_antennas = 0;
        p.iter().tuple_combinations().for_each(|((a, b), (c, d))| {
            let i_diff = (*a as isize) - (*c as isize);
            let j_diff = (*b as isize) - (*d as isize);
            for x in 0..30 {
                let a_1 = ((*a as isize) - x * i_diff, (*b as isize) - x * j_diff);
                let a_2 = ((*c as isize) + x * i_diff, (*d as isize) + x * j_diff);
                try_add_antenna(a_1, &mut new_antennas, &mut a_nodes, size);
                try_add_antenna(a_2, &mut new_antennas, &mut a_nodes, size);
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
