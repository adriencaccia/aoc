use itertools::Itertools;

/// Binary search to find the minimum time to beat the distance, and then calculate the number of
/// ways to beat the distance.
fn ways_to_beat_binary_search(params: (usize, usize)) -> usize {
    let (duration, distance_to_beat) = params;

    let mut min_time_beating = 0;
    let mut max_time_beating = duration;

    while min_time_beating < max_time_beating {
        let time_holding_button = (min_time_beating + max_time_beating) / 2;
        let distance = time_holding_button * (duration - time_holding_button);

        if distance > distance_to_beat {
            max_time_beating = time_holding_button;
        } else {
            min_time_beating = time_holding_button + 1;
        }
    }

    (duration - 2 * min_time_beating) + 1
}

fn parse_input(input: &str) -> (u32, u32) {
    let lines: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let races = lines[0].iter().zip(lines[1].iter());

    let single_race: (usize, usize) =
        races
            .clone()
            .rev()
            .fold((0, 0), |acc, (&duration, &distance_to_beat)| match acc {
                (0, 0) => (duration, distance_to_beat),
                _ => (
                    acc.0 + duration * 10_usize.pow(acc.0.ilog10() + 1),
                    acc.1 + distance_to_beat * 10_usize.pow(acc.1.ilog10() + 1),
                ),
            });

    let part1 = races
        .map(|(&dur, &dist)| ways_to_beat_binary_search((dur, dist)))
        .product::<usize>() as u32;

    let part2 = ways_to_beat_binary_search(single_race) as u32;

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
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 288);
        assert_eq!(part2, 71503);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 140_220);
        assert_eq!(part2, 39_570_185);
    }
}
