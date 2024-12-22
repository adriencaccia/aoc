use rustc_hash::{FxHashMap, FxHashSet};

const CHANGES: usize = 2000;

fn next(secret: u64) -> u64 {
    let mut secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    secret = ((secret * 2048) ^ secret) % 16777216;

    secret
}

pub fn part1(input: &str) -> u64 {
    input.lines().fold(0, |acc, line| {
        let mut secret: u64 = line.parse().unwrap();

        for _ in 0..CHANGES {
            secret = next(secret);
        }

        acc + secret
    })
}

pub fn part2(input: &str) -> u64 {
    let mut bananas_for_sequence = FxHashMap::default();

    input.lines().for_each(|line| {
        let mut seen = FxHashSet::default();
        let mut prices = [0; CHANGES + 1];
        let mut changes = [0; CHANGES + 1];
        let mut secret: u64 = line.parse().unwrap();
        prices[0] = secret % 10;

        for i in 1..=CHANGES {
            secret = next(secret);
            let price = secret % 10;
            prices[i] = price;
            changes[i] = prices[i] as i8 - prices[i - 1] as i8;
            if i > 3 {
                let sequence = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
                if seen.contains(&sequence) {
                    continue;
                } else {
                    bananas_for_sequence.entry(sequence).or_insert(0);
                    *bananas_for_sequence.get_mut(&sequence).unwrap() += price;
                    seen.insert(sequence);
                }
            }
        }
    });

    *bananas_for_sequence.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_PART_1: &str = indoc! {"
        1
        10
        100
        2024
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_PART_1), 37327623);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 17724064040);
    }

    const EXAMPLE_INPUT_PART_2: &str = indoc! {"
        1
        2
        3
        2024
"};

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_PART_2), 23);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1998);
    }
}
