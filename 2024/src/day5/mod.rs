use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

fn parse(input: &str) -> (HashSet<(u8, u8)>, Vec<Vec<u8>>) {
    let mut it = input.split("\n\n");
    let rules = HashSet::from_iter(it.next().unwrap().lines().map(|l| {
        l.split('|')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    let updates = it
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|v| v.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_update_correct_order(update: &[u8], rules: &HashSet<(u8, u8)>) -> bool {
    let len = update.len();
    for i in 0..len - 1 {
        for j in i..len {
            let a = update[i];
            let b = update[j];
            if rules.contains(&(b, a)) {
                return false;
            }
        }
    }

    true
}

pub fn part1(input: &str) -> u16 {
    let (rules, updates) = parse(input);

    updates.into_iter().fold(0, |acc, update| {
        if !is_update_correct_order(&update, &rules) {
            return acc;
        }

        acc + update[update.len() / 2] as u16
    })
}

fn sort_update(update: &mut [u8], rules: &HashSet<(u8, u8)>) {
    update.sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            return Ordering::Less;
        }
        if rules.contains(&(*b, *a)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    })
}

pub fn part2(input: &str) -> u16 {
    let (rules, updates) = parse(input);

    updates.into_iter().fold(0, |acc, mut update| {
        if is_update_correct_order(&update, &rules) {
            return acc;
        }
        sort_update(&mut update, &rules);

        acc + update[update.len() / 2] as u16
    })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
"};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT);

        assert_eq!(part1, 143);
    }

    #[test]
    fn test_part1() {
        let part1 = part1(include_str!("input.txt"));

        assert_eq!(part1, 4578);
    }

    #[test]
    fn test_example_part2() {
        let part2 = part2(EXAMPLE_INPUT);

        assert_eq!(part2, 123);
    }

    #[test]
    fn test_part2() {
        let part2 = part2(include_str!("input.txt"));

        assert_eq!(part2, 6179);
    }
}
