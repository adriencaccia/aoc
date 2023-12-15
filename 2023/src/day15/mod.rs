use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
enum Operation {
    #[display("{label}-")]
    Remove { label: String },
    #[display("{label}={focal_length}")]
    Set { label: String, focal_length: u32 },
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn parse_input(input: &str) -> (u32, u32) {
    let part1 = input.trim().split(',').map(hash).sum();

    let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    for operation in input
        .trim()
        .split(',')
        .map(|s| s.parse::<Operation>().unwrap())
    {
        match operation {
            Operation::Remove { label } => {
                let box_idx = hash(label.as_str()) as usize;
                if let Some((label_position, _)) = boxes[box_idx]
                    .iter()
                    .find_position(|(l, _)| l == label.as_str())
                {
                    boxes[box_idx].remove(label_position);
                }
            }
            Operation::Set {
                label,
                focal_length,
            } => {
                let box_idx = hash(label.as_str()) as usize;
                if let Some((label_position, _)) = boxes[box_idx]
                    .iter()
                    .find_position(|(l, _)| l == label.as_str())
                {
                    boxes[box_idx][label_position].1 = focal_length;
                } else {
                    boxes[box_idx].push((label, focal_length));
                }
            }
        }
    }

    let part2 = boxes
        .into_iter()
        .enumerate()
        .fold(0, |acc, (box_idx, operations)| {
            acc + operations
                .into_iter()
                .enumerate()
                .map(|(slot_idx, (_, focal_length))| {
                    (box_idx as u32 + 1) * (slot_idx as u32 + 1) * focal_length
                })
                .sum::<u32>()
        });
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
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 1320);
        assert_eq!(part2, 145);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 512283);
        assert_eq!(part2, 215827);
    }
}
