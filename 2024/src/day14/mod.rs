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

// sizes for the example input
// const WIDTH = 11;
// const HEIGHT = 7;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const HALF_W: i32 = WIDTH / 2;
const HALF_H: i32 = HEIGHT / 2;

const ROBOTS_SIZE: usize = 500;

pub fn part1(input: &str) -> u32 {
    let robots: ArrayVec<SlowRobot, ROBOTS_SIZE> =
        input.lines().map(|line| line.parse().unwrap()).collect();
    let mut quadrants: [u32; 4] = [0; 4];

    robots.iter().for_each(|robot| {
        let x = (((robot.px + 100 * robot.vx) % WIDTH) + WIDTH) % WIDTH;
        let y = (((robot.py + 100 * robot.vy) % HEIGHT) + HEIGHT) % HEIGHT;

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

fn fake_variance(values: &[u32]) -> u32 {
    let mean = values.iter().sum::<u32>() / values.len() as u32;
    let variance = values.iter().map(|&x| x.abs_diff(mean)).sum();
    variance
}

const VARIANCE_THRESHOLD: u32 = 17_000; // arbitrary threshold
const STEP_START: u32 = 6_500; // arbitrary starting point

pub fn part2(input: &str) -> u32 {
    let mut robots: ArrayVec<SlowRobot, ROBOTS_SIZE> =
        input.lines().map(|line| line.parse().unwrap()).collect();

    robots.iter_mut().for_each(|robot| {
        robot.px = (((robot.px + STEP_START as i32 * robot.vx) % WIDTH) + WIDTH) % WIDTH;
        robot.py = (((robot.py + STEP_START as i32 * robot.vy) % HEIGHT) + HEIGHT) % HEIGHT;
    });

    let mut seconds = STEP_START;
    loop {
        for robot in robots.iter_mut() {
            robot.px = (((robot.px + robot.vx) % WIDTH) + WIDTH) % WIDTH;
            robot.py = (((robot.py + robot.vy) % HEIGHT) + HEIGHT) % HEIGHT;
        }
        seconds += 1;

        let x_positions: ArrayVec<u32, ROBOTS_SIZE> =
            robots.iter().map(|robot| robot.px as u32).collect();
        let y_positions: ArrayVec<u32, ROBOTS_SIZE> =
            robots.iter().map(|robot| robot.py as u32).collect();

        let variance = fake_variance(&x_positions) + fake_variance(&y_positions);

        if variance < VARIANCE_THRESHOLD {
            break;
        }
    }

    seconds
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
