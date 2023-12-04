use std::collections::HashMap;

use itertools::Itertools;

fn parse_input(input: &str) -> (u32, u32) {
    let lines = input.trim().lines();

    let mut scratch_copies: HashMap<usize, u32> = HashMap::new();

    let part1 = lines
        .enumerate()
        .map(|(idx, line)| {
            let (winning, mine) = line
                .split(':')
                .last()
                .unwrap()
                .split('|')
                .map(|numbers| numbers.split_whitespace())
                .collect_tuple()
                .unwrap();

            let number_of_wins: u32 = mine
                .filter(|number| winning.clone().any(|win| &win == number))
                .collect_vec()
                .len()
                .try_into()
                .unwrap();

            let number_of_copies = scratch_copies.entry(idx).or_insert(1).to_owned();
            for i in 0..number_of_wins {
                *scratch_copies.entry(idx + i as usize + 1).or_insert(1) += number_of_copies;
            }

            match number_of_wins {
                0 => 0,
                _ => 2_u32.pow(number_of_wins - 1),
            }
        })
        .sum();

    let part2 = scratch_copies.values().sum();
    (part1, part2)
}

pub fn main() -> (u32, u32) {
    let (part1, part2) = parse_input(include_str!("input.txt"));
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT.trim());

        assert_eq!(part1, 13);
        assert_eq!(part2, 30);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 24706);
        assert_eq!(part2, 13114317);
    }
}
