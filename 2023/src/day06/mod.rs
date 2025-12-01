use itertools::Itertools;

fn solve_quadratic_equation(params: (f64, f64)) -> usize {
    let a = -1.0;
    let (b, c) = params;
    let discriminant = b * b - (4.0 * a * c);
    let sqrt_discriminant = discriminant.sqrt();

    let x1 = ((-(b) + sqrt_discriminant) / (2.0 * a)).ceil() as usize;
    let x2 = ((-(b) - sqrt_discriminant) / (2.0 * a)).floor() as usize;

    x2 - x1 + 1
}

fn parse_input(input: &str) -> (u32, u32) {
    let lines: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.split(':')
                .next_back()
                .unwrap()
                .split_ascii_whitespace()
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
        .map(|(&dur, &dist)| solve_quadratic_equation((dur as f64, -(1.0 + dist as f64))))
        .product::<usize>() as u32;

    let part2 =
        solve_quadratic_equation((single_race.0 as f64, -(1.0 + single_race.1 as f64))) as u32;

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
