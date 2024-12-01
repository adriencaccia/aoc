use itertools::Itertools;
use std::{collections::HashMap, iter::zip};

pub fn part1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .trim()
        .lines()
        .map(|l| {
            let (a, b): (u32, u32) = l
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect_tuple()
                .unwrap();
            (a, b)
        })
        .unzip();

    left.sort();
    right.sort();

    zip(left, right).fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

struct Occurrence {
    left: u32,
    right: u32,
}

pub fn part2(input: &str) -> u32 {
    let mut hash: HashMap<u32, Occurrence> = HashMap::new();

    input.trim().lines().for_each(|l| {
        let (a, b): (u32, u32) = l
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect_tuple()
            .unwrap();

        hash.entry(a)
            .and_modify(|Occurrence { left, right: _ }| *left += 1)
            .or_insert(Occurrence { left: 1, right: 0 });
        hash.entry(b)
            .and_modify(|Occurrence { left: _, right }| *right += 1)
            .or_insert(Occurrence { left: 0, right: 1 });
    });

    hash.iter()
        .fold(0, |acc, (value, occ)| acc + value * occ.left * occ.right)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
"};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT);

        assert_eq!(part1, 11);
    }

    #[test]
    fn test_part1() {
        let part1 = part1(include_str!("input.txt"));

        assert_eq!(part1, 936063);
    }

    #[test]
    fn test_example_part2() {
        let part2 = part2(EXAMPLE_INPUT);

        assert_eq!(part2, 31);
    }

    #[test]
    fn test_part2() {
        let part2 = part2(include_str!("input.txt"));

        assert_eq!(part2, 23150395);
    }
}
