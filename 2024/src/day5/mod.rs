use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

const UPDATE_MAX_SIZE: usize = 23; // real size is 23, we go to 24 to store additional information
const UPDATES_LEN: usize = 187;

fn parse_part1(input: &str) -> (HashSet<(u8, u8)>, [[u8; UPDATE_MAX_SIZE + 1]; UPDATES_LEN]) {
    let mut it = input.split("\n\n");
    let rules = HashSet::from_iter(it.next().unwrap().lines().map(|l| {
        l.split('|')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    let mut updates = [[0; UPDATE_MAX_SIZE + 1]; UPDATES_LEN];
    it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        let mut peekable = l.split(',').peekable();
        let mut j = 0;
        while let Some(v) = peekable.peek() {
            updates[i][j] = v.parse().unwrap();
            j += 1;
            peekable.next();
        }
        updates[i][UPDATE_MAX_SIZE] = updates[i][j / 2]; // store the middle value in the last position
    });

    (rules, updates)
}

fn parse_part2(input: &str) -> (HashSet<(u8, u8)>, [[u8; UPDATE_MAX_SIZE + 1]; UPDATES_LEN]) {
    let mut it = input.split("\n\n");
    let rules = HashSet::from_iter(it.next().unwrap().lines().map(|l| {
        l.split('|')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap()
    }));

    let mut updates = [[0; UPDATE_MAX_SIZE + 1]; UPDATES_LEN];
    it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        let mut peekable = l.split(',').peekable();
        let mut j = 0;
        while let Some(v) = peekable.peek() {
            updates[i][j] = v.parse().unwrap();
            j += 1;
            peekable.next();
        }
        updates[i][UPDATE_MAX_SIZE] = j as u8; // store the length of the update in the last position
    });

    (rules, updates)
}

fn is_update_correct_order(update: &[u8], rules: &HashSet<(u8, u8)>) -> bool {
    for i in 0..UPDATE_MAX_SIZE - 1 {
        let a = update[i];
        for &b in &update[i + 1..UPDATE_MAX_SIZE] {
            if b == 0 {
                break;
            }
            if rules.contains(&(b, a)) {
                return false;
            }
        }
    }

    true
}

pub fn part1(input: &str) -> u16 {
    let (rules, updates) = parse_part1(input);

    updates.into_iter().fold(0, |acc, update| {
        if !is_update_correct_order(&update, &rules) {
            return acc;
        }

        acc + update[UPDATE_MAX_SIZE] as u16
    })
}

fn sort_update(update: &mut [u8], rules: &HashSet<(u8, u8)>) {
    // only sort the first update[UPDATE_MAX_SIZE] elements
    let len = update[UPDATE_MAX_SIZE] as usize;
    update[..len].sort_by(|a, b| {
        if rules.contains(&(*a, *b)) {
            return Ordering::Less;
        }
        if rules.contains(&(*b, *a)) {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
    update[UPDATE_MAX_SIZE] = update[update[UPDATE_MAX_SIZE] as usize / 2];
}

pub fn part2(input: &str) -> u16 {
    let (rules, updates) = parse_part2(input);

    updates.into_iter().fold(0, |acc, mut update| {
        if is_update_correct_order(&update, &rules) {
            return acc;
        }
        sort_update(&mut update, &rules);

        acc + update[UPDATE_MAX_SIZE] as u16
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
