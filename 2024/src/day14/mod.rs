use arrayvec::ArrayVec;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
#[display("p={px},{py} v={vx},{vy}")]
struct SlowRobot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    // Fast parser that assumes valid input format
    #[inline]
    fn parse(s: &str) -> Self {
        // Skip "p=" prefix (2 chars)
        let chars = s.as_bytes();
        let mut pos = 2;

        // Parse px
        let mut px = 0i32;
        while chars[pos] != b',' {
            px = px * 10 + (chars[pos] - b'0') as i32;
            pos += 1;
        }

        // Skip "," (1 char)
        pos += 1;

        // Parse py
        let mut py = 0i32;
        while chars[pos] != b' ' {
            py = py * 10 + (chars[pos] - b'0') as i32;
            pos += 1;
        }

        // Skip " v=" (3 chars)
        pos += 3;

        // Parse vx
        let mut vx = 0i32;
        let mut is_negative = false;
        if chars[pos] == b'-' {
            is_negative = true;
            pos += 1;
        }
        while chars[pos] != b',' {
            vx = vx * 10 + (chars[pos] - b'0') as i32;
            pos += 1;
        }
        if is_negative {
            vx = -vx;
        }

        // Skip "," (1 char)
        pos += 1;

        // Parse vy
        let mut vy = 0i32;
        is_negative = false;
        if chars[pos] == b'-' {
            is_negative = true;
            pos += 1;
        }
        while pos < chars.len() && chars[pos] != b'\n' {
            vy = vy * 10 + (chars[pos] - b'0') as i32;
            pos += 1;
        }
        if is_negative {
            vy = -vy;
        }

        Robot { px, py, vx, vy }
    }
}

// sizes for the example input
// const WIDTH = 11;
// const HEIGHT = 7;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const HALF_W: i32 = WIDTH / 2;
const HALF_H: i32 = HEIGHT / 2;

const ROBOTS_SIZE: usize = 500;

pub fn part1(input: &str) -> u32 {
    let robots: ArrayVec<Robot, ROBOTS_SIZE> = input.lines().map(Robot::parse).collect();
    let mut quadrants: [u32; 4] = [0; 4];

    robots.iter().for_each(|robot| {
        let x = (robot.px + 100 * robot.vx).rem_euclid(WIDTH);
        let y = (robot.py + 100 * robot.vy).rem_euclid(HEIGHT);

        if x < HALF_W && y < HALF_H {
            quadrants[0] += 1;
        } else if x > HALF_W && y < HALF_H {
            quadrants[1] += 1;
        } else if x < HALF_W && y > HALF_H {
            quadrants[2] += 1;
        } else if x > HALF_W && y > HALF_H {
            quadrants[3] += 1;
        }
    });

    quadrants.iter().product()
}

// no need to use all 500 robots since we are checking the variance
const SAMPLED_SIZE: usize = 128;
const VARIANCE_THRESHOLD: u32 = 2300; // arbitrary threshold
const WIDTH_INVERSE_IN_CHINESE_REMAINDER_THEOREM_WITH_HEIGHT: u32 = 51;

fn fake_variance(values: &[u32]) -> u32 {
    let mean = values.iter().sum::<u32>() / SAMPLED_SIZE as u32;
    let variance = values.iter().map(|&x| x.abs_diff(mean)).sum();
    variance
}

pub fn part2(input: &str) -> u32 {
    let mut robots: ArrayVec<Robot, SAMPLED_SIZE> =
        input.lines().take(SAMPLED_SIZE).map(Robot::parse).collect();

    let mut bx = 0;
    let mut by = 0;

    for seconds in 1..=(HEIGHT as usize) {
        for robot in robots.iter_mut() {
            robot.px = (robot.px + robot.vx).rem_euclid(WIDTH);
            robot.py = (robot.py + robot.vy).rem_euclid(HEIGHT);
        }

        let x_positions: ArrayVec<u32, SAMPLED_SIZE> =
            robots.iter().map(|robot| robot.px as u32).collect();
        if fake_variance(&x_positions) < VARIANCE_THRESHOLD {
            bx = seconds;
            if by != 0 {
                break;
            }
        }

        let y_positions: ArrayVec<u32, SAMPLED_SIZE> =
            robots.iter().map(|robot| robot.py as u32).collect();
        if fake_variance(&y_positions) < VARIANCE_THRESHOLD {
            by = seconds;
            if bx != 0 {
                break;
            }
        }
    }

    // see https://www.reddit.com/r/adventofcode/comments/1he0asr/2024_day_14_part_2_why_have_fun_with_image/
    bx as u32
        + (WIDTH_INVERSE_IN_CHINESE_REMAINDER_THEOREM_WITH_HEIGHT * (by as u32 - bx as u32))
            .rem_euclid(HEIGHT as u32)
            * WIDTH as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
"};

    #[test]
    #[ignore]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 12);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 211692000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 6587);
    }
}
