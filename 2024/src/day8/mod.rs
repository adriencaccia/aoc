const GRID_SIZE: usize = 50;
const ASCII_RANGE: usize = 128;
const POS_SIZE: usize = 4; // each antenna has at most 4 occurrences

#[derive(Copy, Clone)]
struct CharInfo {
    positions: [(usize, usize); POS_SIZE],
    count: u8,
}

impl Default for CharInfo {
    #[inline(always)]
    fn default() -> Self {
        Self {
            positions: [(0, 0); POS_SIZE],
            count: 0,
        }
    }
}

type CharData = [CharInfo; ASCII_RANGE];

#[inline(always)]
fn parse(input: &str) -> (CharData, usize) {
    let mut char_data: CharData = [CharInfo::default(); ASCII_RANGE];
    let mut size = 0;

    for (i, line) in input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|l| !l.is_empty())
        .enumerate()
    {
        for (j, &c) in line.iter().enumerate() {
            if c != b'.' {
                let info = &mut char_data[c as usize];
                if info.count < POS_SIZE as u8 {
                    info.positions[info.count as usize] = (i, j);
                    info.count += 1;
                }
            }
        }
        size = i + 1;
    }

    (char_data, size)
}

pub fn part1(input: &str) -> u32 {
    let (char_data, size) = parse(input);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];
    let mut total = 0;

    for char_info in char_data.iter() {
        for i in 0..char_info.count as usize {
            for j in (i + 1)..char_info.count as usize {
                let (a, b) = char_info.positions[i];
                let (c, d) = char_info.positions[j];
                let i_diff = (a as isize) - (c as isize);
                let j_diff = (b as isize) - (d as isize);
                let i = (a as isize) - 2 * i_diff;
                let j = (b as isize) - 2 * j_diff;
                if i >= 0
                    && i < size as isize
                    && j >= 0
                    && j < size as isize
                    && unsafe { !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) }
                {
                    total += 1;
                    a_nodes[i as usize][j as usize] = true;
                }
                let i = (c as isize) + 2 * i_diff;
                let j = (d as isize) + 2 * j_diff;
                if i >= 0
                    && i < size as isize
                    && j >= 0
                    && j < size as isize
                    && unsafe { !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) }
                {
                    total += 1;
                    a_nodes[i as usize][j as usize] = true;
                }
            }
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let (char_data, size) = parse(input);
    let mut a_nodes = [[false; GRID_SIZE]; GRID_SIZE];
    let mut total = 0;

    for char_info in char_data.iter() {
        for i in 0..char_info.count as usize {
            for j in (i + 1)..char_info.count as usize {
                let (a, b) = char_info.positions[i];
                let (c, d) = char_info.positions[j];
                let i_diff = (a as isize) - (c as isize);
                let j_diff = (b as isize) - (d as isize);
                let mut i = a as isize;
                let mut j = b as isize;
                while i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                    if unsafe { !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) } {
                        total += 1;
                        a_nodes[i as usize][j as usize] = true;
                    }
                    i -= i_diff;
                    j -= j_diff;
                }
                let mut i = c as isize;
                let mut j = d as isize;
                while i >= 0 && i < size as isize && j >= 0 && j < size as isize {
                    if unsafe { !a_nodes.get_unchecked(i as usize).get_unchecked(j as usize) } {
                        total += 1;
                        a_nodes[i as usize][j as usize] = true;
                    }
                    i += i_diff;
                    j += j_diff;
                }
            }
        }
    }

    total
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
