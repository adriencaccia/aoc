use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input.trim_ascii().split(",").fold(0, |acc, range| {
        let (start, end) = range.split("-").collect_tuple().unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();

        let start_pow = ((start as f32).log10() / 2f32).ceil() as u32;
        let mut pattern = start / 10usize.pow(start_pow);
        if pattern == 0 {
            pattern = 1;
        }
        let mut counter = 0;
        loop {
            let test = pattern * 10usize.pow(pattern.ilog10() + 1) + pattern;
            if start <= test && test <= end {
                counter += test
            }
            if test > end {
                break;
            }
            pattern += 1;
        }
        acc + counter
    })
}

pub fn part2(input: &str) -> usize {
    input.trim_ascii().split(",").fold(0, |acc, range| {
        let (start, end) = range.split("-").collect_tuple().unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();

        let mut invalid_ids = HashSet::new();

        let mut n = start;
        while n <= end {
            let digits = n.ilog10() + 1;
            let next_pow = 10usize.pow(digits);
            let limit = (end + 1).min(next_pow);

            // check all numbers from n to limit
            for i in n..limit {
                let i_digits = i.ilog10() + 1;

                // try patterns of length 1 to i_digits / 2
                for pattern_len in 1..=(i_digits / 2) {
                    // only check the pattern if it divides evenly into the number of digits of i
                    if i_digits.is_multiple_of(pattern_len) {
                        let pattern = i / 10usize.pow(i_digits - pattern_len);
                        let mut test = 0;
                        for _ in 0..(i_digits / pattern_len) {
                            test = test * 10usize.pow(pattern_len) + pattern;
                        }
                        if test == i {
                            invalid_ids.insert(i);
                            break;
                        }
                    }
                }
            }
            n = limit;
        }

        acc + invalid_ids.iter().sum::<usize>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 9188031749);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 11323661261);
    }
}
