struct Claw {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Claw {
    // Fast parser that assumes valid input format
    #[inline]
    fn parse(s: &str) -> Self {
        // Skip "Button A: X+" prefix (12 chars)
        let chars = s.as_bytes();
        let mut pos = 12;

        // Parse ax
        let mut ax = 0i64;
        while chars[pos] != b',' {
            ax = ax * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }

        // Skip ", Y+" (4 chars)
        pos += 4;

        // Parse ay
        let mut ay = 0i64;
        while chars[pos] != b'\n' {
            ay = ay * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }

        // Skip "\nButton B: X+" (13 chars)
        pos += 13;

        // Parse bx
        let mut bx = 0i64;
        while chars[pos] != b',' {
            bx = bx * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }

        // Skip ", Y+" (4 chars)
        pos += 4;

        // Parse by
        let mut by = 0i64;
        while chars[pos] != b'\n' {
            by = by * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }

        // Skip "\nPrize: X=" (10 chars)
        pos += 10;

        // Parse px
        let mut px = 0i64;
        let mut px_negative = false;
        if chars[pos] == b'-' {
            px_negative = true;
            pos += 1;
        }
        while chars[pos] != b',' {
            px = px * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }
        if px_negative {
            px = -px;
        }

        // Skip ", Y=" (4 chars)
        pos += 4;

        // Parse py
        let mut py = 0i64;
        while pos < chars.len() {
            py = py * 10 + (chars[pos] - b'0') as i64;
            pos += 1;
        }

        Claw {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    }
}

fn inner(input: &str, offset: i64) -> i64 {
    input.trim_ascii_end().split("\n\n").fold(0, |acc, claw| {
        let mut claw = Claw::parse(claw);
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
