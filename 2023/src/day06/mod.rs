use itertools::Itertools;

fn parse_input(input: &str) -> (u32, u32) {
    let lines: Vec<Vec<u32>> = input
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

    let part1: u32 = races
        .map(|(duration, distance_to_beat)| {
            let ways_to_beat = (0..=*duration)
                .collect_vec()
                .iter()
                .map(|time_holding_button| {
                    let distance = time_holding_button * (duration - time_holding_button);
                    distance > *distance_to_beat
                })
                .filter(|w| *w)
                .collect_vec();

            ways_to_beat.len() as u32
        })
        .product();

    let part2 = 0;

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
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 140220);
        assert_eq!(part2, 0);
    }
}
