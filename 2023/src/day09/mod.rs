use itertools::Itertools;

fn parse_input(input: &str) -> (i32, i32) {
    let lines: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut part1 = 0;
    for mut line in lines.clone() {
        let mut acc = 0;
        let mut start_idx = 1;
        while *line.last().unwrap() != 0 {
            let value_to_add_end = line[line.len() - 1];
            let mut value_to_subtract = line[start_idx - 1];
            (start_idx..line.len()).for_each(|idx| {
                let new_value = line[idx] - value_to_subtract;
                value_to_subtract = line[idx];
                line[idx] = new_value;
            });
            start_idx += 1;
            acc += value_to_add_end;
        }
        part1 += acc;
    }

    let mut part2 = 0;
    for mut line in lines {
        let mut start_idx = 1;
        let mut start_values: Vec<i32> = Vec::new();
        while *line.last().unwrap() != 0 {
            start_values.push(line[start_idx - 1]);
            let mut value_to_subtract = line[start_idx - 1];
            (start_idx..line.len()).for_each(|idx| {
                let new_value = line[idx] - value_to_subtract;
                value_to_subtract = line[idx];
                line[idx] = new_value;
            });
            start_idx += 1;
        }
        part2 += start_values.iter().rev().fold(0, |acc, n| n - acc);
    }
    (part1, part2)
}

pub fn main() -> (i32, i32) {
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
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 114);
        assert_eq!(part2, 2);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 1868368343);
        assert_eq!(part2, 1022);
    }
}
