use regex::Regex;
use std::sync::LazyLock;

static MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((?P<first>\d+),(?P<second>\d+)\)").unwrap());
static MUL_INSTRUCTIONS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(mul\((?P<first>\d+),(?P<second>\d+)\))|(?P<do>do\(\))|(?P<dont>don't\(\))")
        .unwrap()
});

fn add_mul(sum: &mut u32, capture: regex::Captures<'_>) {
    let first: u32 = capture.name("first").unwrap().as_str().parse().unwrap();
    let second: u32 = capture.name("second").unwrap().as_str().parse().unwrap();
    *sum += first * second;
}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for capture in MUL_REGEX.captures_iter(input) {
        add_mul(&mut sum, capture);
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut enabled = true;

    for capture in MUL_INSTRUCTIONS_REGEX.captures_iter(input) {
        if capture.name("do").is_some() {
            if !enabled {
                enabled = true;
            }
            continue;
        }
        if capture.name("dont").is_some() {
            if enabled {
                enabled = false;
            }
            continue;
        }
        if !enabled {
            continue;
        }

        add_mul(&mut sum, capture);
    }

    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT_PART_1: &str = indoc! {"
    xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"};
    const EXAMPLE_INPUT_PART_2: &str = indoc! {"
    xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT_PART_1);

        assert_eq!(part1, 161);
    }

    #[test]
    fn test_part1() {
        let part1 = part1(include_str!("input.txt"));

        assert_eq!(part1, 169021493);
    }

    #[test]
    fn test_example_part2() {
        let part2 = part2(EXAMPLE_INPUT_PART_2);

        assert_eq!(part2, 48);
    }

    #[test]
    fn test_part2() {
        let part2 = part2(include_str!("input.txt"));

        assert_eq!(part2, 111762583);
    }
}
