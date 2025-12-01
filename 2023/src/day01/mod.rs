fn parse_input(input: &str) -> (u32, u32) {
    let (part1, part2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|char| char.to_digit(10));
            let first = digits.next().unwrap();

            let part1 = match digits.next_back() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            };

            let numbers: Vec<String> = vec![
                "one".into(),
                "two".into(),
                "three".into(),
                "four".into(),
                "five".into(),
                "six".into(),
                "seven".into(),
                "eight".into(),
                "nine".into(),
            ];
            let first = find_in_string(line, &numbers);

            let reversed_line = line.chars().rev().collect::<String>();
            let reversed_numbers: Vec<String> = numbers
                .into_iter()
                .map(|number| number.chars().rev().collect::<String>())
                .collect();
            let last = find_in_string(&reversed_line, &reversed_numbers);

            let part2 = first * 10 + last;

            (part1, part2)
        })
        .unzip();

    (part1.iter().sum(), part2.iter().sum())
}

fn find_in_string(line: &str, numbers: &[String]) -> u32 {
    for (idx, char) in line.chars().enumerate() {
        if let Some(digit) = char.to_digit(10) {
            return digit;
        }

        for (value, number) in numbers.iter().enumerate() {
            if line.get(0..idx + 1).unwrap().contains(number) {
                return value as u32 + 1;
            }
        }
    }

    panic!("No digit or number found in {}", line)
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

    #[test]
    fn test_example_part1() {
        let (part1, _part2) = parse_input(indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "});

        assert_eq!(part1, 142);
    }

    #[test]
    fn test_example_part2() {
        let (_part1, part2) = parse_input(
            // ! Added a 1 to the second line to make the part1 algo work
            indoc! {"
                two1nine
                eightwo1three
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen
            "},
        );

        assert_eq!(part2, 281);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 54644);
        assert_eq!(part2, 53348);
    }
}
