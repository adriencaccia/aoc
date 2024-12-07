pub fn part1(input: &str) -> u32 {
    0
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        REPLACE
        ME
        WITH THE EXAMPLE
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 0);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 0);
    }
}
