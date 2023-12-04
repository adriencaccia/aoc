use std::collections::HashMap;

use regex::Regex;

fn parse_input(input: &str) -> (u32, u32) {
    let matrix: Vec<&str> = input.lines().collect();

    let height = matrix.len();
    let width = matrix[0].len();

    let mut part1 = 0;

    let numbers_regex = Regex::new(r"(\d+)").unwrap();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..height {
        let captures = numbers_regex.captures_iter(matrix[i]);
        for cap in captures {
            let number_match = cap.get(1).unwrap();
            let range = number_match.range();
            let number: u32 = number_match.as_str().parse().unwrap();
            let start = if range.start == 0 { 0 } else { range.start - 1 };
            let end = if range.end == width {
                range.end
            } else {
                range.end + 1
            };
            let range_to_match = start..end;
            let mut indices = Vec::new();
            if i != 0 {
                indices.extend(range_to_match.clone().map(|y| (i - 1, y)));
            }
            if i + 1 < height {
                indices.extend(range_to_match.clone().map(|y| (i + 1, y)));
            }
            indices.push((i, range_to_match.start));
            indices.push((i, range_to_match.end - 1));

            let has_symbol = indices
                .into_iter()
                .map(|(x, y)| {
                    let char = matrix[x].chars().nth(y).unwrap();
                    if char == '*' {
                        gears.entry((x, y)).or_default().push(number);

                        return true;
                    }
                    !char.is_ascii_digit() && char != '.'
                })
                .any(|b| b);

            if has_symbol {
                part1 += number;
            }
        }
    }

    let part2 = gears
        .values()
        .map(|parts| {
            if parts.len() != 2 {
                return 0;
            }

            parts[0] * parts[1]
        })
        .sum();

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
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 4361);
        assert_eq!(part2, 467835);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 527364);
        assert_eq!(part2, 79026871);
    }
}
