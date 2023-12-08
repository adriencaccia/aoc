use std::collections::HashMap;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{id} = ({left}, {right})")]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (u32, u32) {
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

    let part2 = 0;
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
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 6);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 16343);
        assert_eq!(part2, 0);
    }
}
