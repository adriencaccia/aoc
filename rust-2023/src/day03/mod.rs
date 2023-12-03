use regex::Regex;

fn get_char_at_index(input_string: &str, index: usize) -> Option<char> {
    input_string.as_bytes().get(index).map(|byte| *byte as char)
}

fn is_symbol(char: char) -> bool {
    !char.is_ascii_digit() && char != '.'
}

fn parse_input() -> u32 {
    let matrix: Vec<&str> = include_str!("input.txt").lines().collect();

    let height = matrix.len();
    let width = matrix[0].len();

    let mut sum = 0;

    let numbers_regex = Regex::new(r"(\d+)").unwrap();

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
                    .map_or(false, |sub| sub.chars().any(is_symbol));
            let is_symbol_in_lower_string = i + 1 < height
                && matrix[i + 1]
                    .get(range_to_match.clone())
                    .map_or(false, |sub| sub.chars().any(is_symbol));
            let is_symbol_left = range.start != range_to_match.clone().start
                && get_char_at_index(matrix[i], range_to_match.clone().start)
                    .map_or(false, is_symbol);
            let is_symbol_right = range.end != range_to_match.clone().end
                && get_char_at_index(matrix[i], range_to_match.end - 1).map_or(false, is_symbol);

            if is_symbol_in_lower_string
                || is_symbol_in_upper_string
                || is_symbol_left
                || is_symbol_right
            {
                sum += number;
            }
        }
    }

    sum
}

pub fn main() -> (u32, u32) {
    let part1 = parse_input();
    println!("part1 {}", part1);

    (part1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (part1, part2) = main();

        assert_eq!(part1, 527364);
        assert_eq!(part2, 0);
    }
}
