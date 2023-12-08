use std::collections::HashMap;

use gcd::Gcd;
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{id} = ({left}, {right})")]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (u32, usize) {
    let mut lines = input.trim().lines();
    let instructions = lines.next().unwrap().chars().collect_vec();
    lines.next();
    let mut network: HashMap<String, Node> = HashMap::new();

    for line in lines {
        let node: Node = line.parse().unwrap();
        network.insert(node.id.clone(), node);
    }
    let mut current_id: String = "AAA".into();
    let mut part1: u32 = 0;
    loop {
        if current_id == "ZZZ" {
            break;
        }
        let instruction = instructions[part1 as usize % (instructions.len())];
        part1 += 1;
        let current_node = network.get(&current_id).unwrap();
        current_id = match instruction {
            'L' => current_node.left.clone(),
            'R' => current_node.right.clone(),
            _ => "".into(),
        }
    }

    let mut ghost_ids = network
        .keys()
        .filter(|&id| id.ends_with('A'))
        .map(|id| id.to_owned())
        .collect_vec();
    let mut z_indexes: HashMap<usize, usize> = (0..ghost_ids.len()).map(|idx| (idx, 0)).collect();
    let mut step_idx = 0;
    loop {
        ghost_ids.iter().enumerate().for_each(|(idx, id)| {
            if id.ends_with('Z') {
                z_indexes.entry(idx).and_modify(|z_idx| {
                    if *z_idx == 0 {
                        *z_idx = step_idx;
                    }
                });
            }
        });
        if z_indexes.values().filter(|&v| *v != 0).collect_vec().len() == ghost_ids.len() {
            break;
        }
        let instruction = instructions[step_idx % (instructions.len())];
        step_idx += 1;
        ghost_ids = ghost_ids
            .iter()
            .map(|id| {
                let current_node = network.get(id).unwrap();
                match instruction {
                    'L' => current_node.left.clone(),
                    'R' => current_node.right.clone(),
                    _ => "".into(),
                }
            })
            .collect_vec();
    }
    // `lcm` (https://en.wikipedia.org/wiki/Least_common_multiple#Using_the_greatest_common_divisor) of all members of `z_indexes`
    let part2 = z_indexes.values().fold(1, |acc, &n| acc * (n / n.gcd(acc)));
    (part1, part2)
}

pub fn main() -> (u32, usize) {
    let (part1, part2) = parse_input(include_str!("input.txt"));
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn test_example() {
        let (part1, _) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 6);
    }

    #[test]
    fn test_example_part2() {
        // the last two lines have been added to make part1 not error out
        let (_, part2) = parse_input(indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            AAA = (ZZZ, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "});

        assert_eq!(part2, 6);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 16343);
        assert_eq!(part2, 15_299_095_336_639);
    }
}
