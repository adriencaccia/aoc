use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
#[display(
    "Button A: X+{ax}, Y+{ay}
Button B: X+{bx}, Y+{by}
Prize: X={px}, Y={py}"
)]
struct Claw {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn inner(input: &str, offset: i64) -> i64 {
    input.trim_ascii_end().split("\n\n").fold(0, |acc, claw| {
        let mut claw: Claw = claw.parse().unwrap();
        claw.px += offset;
        claw.py += offset;

        let n = (claw.px * claw.by - claw.py * claw.bx) / (claw.ax * claw.by - claw.ay * claw.bx);
        let m = (claw.px * claw.ay - claw.py * claw.ax) / (claw.bx * claw.ay - claw.by * claw.ax);

        if n * claw.ax + m * claw.bx == claw.px && n * claw.ay + m * claw.by == claw.py {
            acc + n * 3 + m
        } else {
            acc
        }
    })
}

pub fn part1(input: &str) -> i64 {
    inner(input, 0)
}

pub fn part2(input: &str) -> i64 {
    inner(input, 10_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 480);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 37686);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 875318608908);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 77204516023437);
    }
}
