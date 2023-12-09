use itertools::Itertools;

fn get_extrapolated_value_from_line(mut line: Vec<i32>, part2: bool) -> i32 {
    let mut start_idx = 1;
    let mut values_to_sum: Vec<i32> = Vec::new();
    while *line.last().unwrap() != 0 {
        match part2 {
            false => values_to_sum.push(line[line.len() - 1]),
            true => values_to_sum.push(line[start_idx - 1]),
        };
        let mut value_to_subtract = line[start_idx - 1];
        (start_idx..line.len()).for_each(|idx| {
            let new_value = line[idx] - value_to_subtract;
            value_to_subtract = line[idx];
            line[idx] = new_value;
        });
        start_idx += 1;
    }

    match part2 {
        false => values_to_sum.iter().sum(),
        true => values_to_sum.iter().rev().fold(0, |acc, n| n - acc),
    }
}

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

    let part1 = lines
        .clone()
        .iter()
        .map(|line| get_extrapolated_value_from_line(line.clone(), false))
        .sum();

    let part2 = lines
        .clone()
        .iter()
        .map(|line| get_extrapolated_value_from_line(line.clone(), true))
        .sum();

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
