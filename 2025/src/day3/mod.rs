pub fn part1(input: &str) -> u32 {
    input.trim_ascii().lines().fold(0, |acc, line| {
        let mut a = 0;
        let mut b = 0;
        let bytes: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

        for i in 0..bytes.len() {
            let ch = bytes[i];
            if i < bytes.len() - 1 && ch > a {
                a = ch;
                b = bytes[i + 1];
            } else if ch > b {
                b = ch;
            }
        }

        acc + (a as u32) * 10 + (b as u32)
    })
}

const LEN: usize = 12;

pub fn part2(input: &str) -> usize {
    input.trim_ascii().lines().fold(0, |acc, line| {
        let bytes: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
        let n = bytes.len();

        let mut joltage: [(u8, u8); LEN] = [(0, 0); LEN];

        // for each position, find the maximum digit that can be placed there
        // this is the highest digit with its index greater than the previous position's index
        // and low enough to leave room for the remaining positions
        for i in 0..LEN {
            let start = if i == 0 {
                0
            } else {
                joltage[i - 1].1 as usize + 1
            };

            for (j, &ch) in bytes
                .iter()
                .enumerate()
                .skip(start)
                .take(n - (LEN - 1 - i) - start)
            {
                if ch > joltage[i].0 {
                    joltage[i].0 = ch;
                    joltage[i].1 = j as u8;
                }
            }
        }

        acc + joltage
            .iter()
            .fold(0, |prod, &(ch, _)| prod * 10 + (ch as usize))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 357);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 16842);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 3121910778619);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 167523425665348);
    }
}
