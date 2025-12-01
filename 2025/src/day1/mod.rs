pub fn part1(input: &str) -> u32 {
    let mut dial: i32 = 50;
    input.trim_ascii().lines().fold(0, |acc, line| {
        let (char, step_str) = line.split_at(1);
        let step: i32 = step_str.parse().unwrap();
        dial = match char {
            "L" => dial - step,
            "R" => dial + step,
            _ => panic!("never happens"),
        }
        .rem_euclid(100);

        if dial == 0 { acc + 1 } else { acc }
    })
}

pub fn part2(input: &str) -> u32 {
    let mut dial = 50;
    input.trim_ascii().lines().fold(0, |acc, line| {
        let (char, step_str) = line.split_at(1);
        let step: i32 = step_str.parse().unwrap();
        let end = match char {
            "L" => dial - step,
            "R" => dial + step,
            _ => panic!("never happens"),
        };

        let n_clicks = match char {
            "L" => (dial - 1).div_euclid(100) - (end - 1).div_euclid(100),
            "R" => end.div_euclid(100),
            _ => panic!("never happens"),
        } as u32;
        dial = end.rem_euclid(100);
        acc + n_clicks
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 984);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 5657);
    }
}
