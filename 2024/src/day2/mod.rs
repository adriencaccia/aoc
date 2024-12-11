use itertools::Itertools;

enum Trend {
    Incr,
    Decr,
}

impl Trend {
    fn from_pair(a: u32, b: u32) -> Self {
        if a > b {
            return Trend::Decr;
        }
        Trend::Incr
    }
}

const MAX_DIFF: u32 = 3;

fn is_line_safe(line: &str) -> bool {
    let mut trend: Option<Trend> = None;
    line.split_ascii_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .tuple_windows()
        .all(|(a, b)| {
            let diff = a.abs_diff(b);

            if !(1..=MAX_DIFF).contains(&diff) {
                return false;
            }

            match trend {
                None => {
                    trend = Some(Trend::from_pair(a, b));
                    true
                }
                Some(Trend::Decr) => a > b,
                Some(Trend::Incr) => a < b,
            }
        })
}

pub fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .fold(0, |sum, l| match is_line_safe(l) {
            false => sum,
            true => sum + 1,
        })
}

pub fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .fold(0, |sum, l| match is_line_safe(l) {
            true => sum + 1,
            false => {
                let l_length = l.split_ascii_whitespace().collect_vec().len();

                match (0..l_length).any(|v| {
                    let mut new_line = l.split_ascii_whitespace().collect_vec();
                    new_line.remove(v);
                    is_line_safe(&new_line.join(" "))
                }) {
                    true => sum + 1,
                    false => sum,
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
"};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT);

        assert_eq!(part1, 2);
    }

    #[test]
    fn test_part1() {
        let part1 = part1(include_str!("input.txt"));

        assert_eq!(part1, 479);
    }

    #[test]
    fn test_example_part2() {
        let part2 = part2(EXAMPLE_INPUT);

        assert_eq!(part2, 4);
    }

    #[test]
    fn test_part2() {
        let part2 = part2(include_str!("input.txt"));

        assert_eq!(part2, 531);
    }
}
