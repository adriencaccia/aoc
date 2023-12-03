use std::collections::HashMap;

use regex::Regex;

fn get_char_at_index(input_string: &str, index: usize) -> Option<char> {
    input_string.as_bytes().get(index).map(|byte| *byte as char)
}

fn is_symbol(char: char) -> bool {
    !char.is_ascii_digit() && char != '.'
}

fn parse_input() -> (u32, u32) {
    let matrix: Vec<&str> = include_str!("input.txt").lines().collect();

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

            let is_symbol_in_upper_string = i != 0
                && matrix[i - 1]
                    .get(range_to_match.clone())
                    .map_or(false, |sub| {
                        for (idx, char) in sub.char_indices() {
                            if char == '*' {
                                gears
                                    .entry((i - 1, range_to_match.start + idx))
                                    .or_default()
                                    .push(number);
                            }
                        }

                        sub.chars().any(is_symbol)
                    });
            let is_symbol_in_lower_string = i + 1 < height
                && matrix[i + 1]
                    .get(range_to_match.clone())
                    .map_or(false, |sub| {
                        for (idx, char) in sub.char_indices() {
                            if char == '*' {
                                gears
                                    .entry((i + 1, range_to_match.start + idx))
                                    .or_default()
                                    .push(number);
                            }
                        }

                        sub.chars().any(is_symbol)
                    });
            let is_symbol_left = range.start != range_to_match.start
                && get_char_at_index(matrix[i], range_to_match.start).map_or(false, |char| {
                    if char == '*' {
                        gears
                            .entry((i, range_to_match.start))
                            .or_default()
                            .push(number);
                    }
                    is_symbol(char)
                });
            let is_symbol_right = range.end != range_to_match.end
                && get_char_at_index(matrix[i], range_to_match.end - 1).map_or(false, |char| {
                    if char == '*' {
                        gears
                            .entry((i, range_to_match.end - 1))
                            .or_default()
                            .push(number);
                    }
                    is_symbol(char)
                });

            if is_symbol_in_lower_string
                || is_symbol_in_upper_string
                || is_symbol_left
                || is_symbol_right
            {
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
    let (part1, part2) = parse_input();
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, part2) = main();

        assert_eq!(part1, 527364);
        assert_eq!(part2, 79026871);
    }
}
