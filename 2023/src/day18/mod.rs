use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Debug)]
#[display("{direction} {meters} ({color})")]
struct Instruction {
    direction: Direction,
    meters: usize,
    color: String,
}

#[derive(Display, FromStr, Eq, PartialEq, Debug)]
enum Direction {
    #[display("U")]
    Up,
    #[display("D")]
    Down,
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

/// Calculates the inner area of the polygon with the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula),
/// then adds the perimeter thanks to [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
fn area_from_ins<I>(ins: I) -> usize
where
    I: IntoIterator<Item = Instruction>,
{
    let mut point = (0_isize, 0_isize);
    let (inner_area, perimeter) = ins.into_iter().fold((0_isize, 0), |(acc, perim), ins| {
        let x1 = point.0;
        let y1 = point.1;
        match ins.direction {
            Direction::Right => point.1 += ins.meters as isize,
            Direction::Down => point.0 += ins.meters as isize,
            Direction::Left => point.1 -= ins.meters as isize,
            Direction::Up => point.0 -= ins.meters as isize,
        };

        (acc + (x1 * point.1 - point.0 * y1), perim + ins.meters)
    });

    (inner_area.unsigned_abs() / 2) + (perimeter / 2) + 1
}

fn parse_input(input: &str) -> (usize, usize) {
    let ins_p1 = input
        .trim()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap());

    let ins_p2 = ins_p1.clone().map(|ins| {
        let direction = match ins.color.chars().last() {
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            _ => unreachable!(),
        };
        let meters =
            usize::from_str_radix(&ins.color.as_str()[1..ins.color.len() - 1], 16).unwrap();

        Instruction {
            color: ins.color,
            direction,
            meters,
        }
    });

    let part1 = area_from_ins(ins_p1);
    let part2 = area_from_ins(ins_p2);
    (part1, part2)
}

pub fn main() -> (usize, usize) {
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
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 62);
        assert_eq!(part2, 952408144115);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 106459);
        assert_eq!(part2, 63806916814808);
    }
}
