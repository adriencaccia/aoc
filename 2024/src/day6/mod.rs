const GRID_SIZE: usize = 130;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    #[inline(always)]
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[inline(always)]
    fn advance(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up if pos.0 != 0 => Some((pos.0 - 1, pos.1)),
            Direction::Right if pos.1 != GRID_SIZE - 1 => Some((pos.0, pos.1 + 1)),
            Direction::Down if pos.0 != GRID_SIZE - 1 => Some((pos.0 + 1, pos.1)),
            Direction::Left if pos.1 != 0 => Some((pos.0, pos.1 - 1)),
            _ => None,
        }
    }

    #[inline(always)]
    fn to_bit(self) -> i32 {
        match self {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }
}

const NOT_VISITED: i32 = 0;
const UP: i32 = 1;
const RIGHT: i32 = 1 << 2;
const DOWN: i32 = 1 << 3;
const LEFT: i32 = 1 << 4;

// TODO: store more information in the grid using bitset operations

pub fn part1(input: &str) -> u16 {
    let mut grid = [[b'0'; GRID_SIZE]; GRID_SIZE];
    let mut visits = [[false; GRID_SIZE]; GRID_SIZE];
    let mut pos = (0, 0);
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'^' {
                visits[i][j] = true;
                pos = (i, j);
            }
        });
    });

    let mut sum = 1;
    let mut dir = Direction::Up;
    let mut previous_pos = pos;
    loop {
        match grid[pos.0][pos.1] {
            b'#' => {
                dir = dir.turn();
                pos = previous_pos;
            }
            // we are outside, useful only for example input
            b'0' => {
                break;
            }
            _ => {
                if !visits[pos.0][pos.1] {
                    sum += 1;
                    visits[pos.0][pos.1] = true;
                }
            }
        }
        if let Some(new_pos) = dir.advance(pos) {
            previous_pos = pos;
            pos = new_pos;
        } else {
            break;
        }
    }
    sum
}

pub fn part2(input: &str) -> u16 {
    let mut grid = [[b'0'; GRID_SIZE]; GRID_SIZE];
    let mut visits = [[NOT_VISITED; GRID_SIZE]; GRID_SIZE];
    let mut pos = (0, 0);
    input.lines().enumerate().for_each(|(i, line)| {
        line.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'^' {
                visits[i][j] = UP;
                pos = (i, j);
            }
        });
    });

    let mut sum = 0;
    let mut path = Vec::with_capacity(5_000);
    path.push((pos, Direction::Up));
    visit_grid(&mut path, &grid, visits);
    for i in 2..path.len() {
        let (pos, dir) = &path[i];
        // add an obstacle on the way, only if it is not in the previous path
        if visits[pos.0][pos.1] != 0 {
            continue;
        }

        visits[pos.0][pos.1] |= dir.to_bit();
        let mut new_grid = grid;
        new_grid[pos.0][pos.1] = b'#';

        // create sub path and turn in front of new obstacle
        let sub_path = &path[0..i];
        let mut sub_path = sub_path.to_vec();
        sub_path.push((path[i - 1].0, dir.turn()));

        let is_cycle = visit_grid(&mut sub_path, &new_grid, visits);
        if is_cycle {
            sum += 1;
        }
    }
    sum
}

fn visit_grid(
    path: &mut Vec<((usize, usize), Direction)>,
    grid: &[[u8; GRID_SIZE]; GRID_SIZE],
    mut visits: [[i32; GRID_SIZE]; GRID_SIZE],
) -> bool {
    let (mut pos, mut dir) = path.last().unwrap();
    let mut previous_pos = pos;
    loop {
        match grid[pos.0][pos.1] {
            b'#' => {
                dir = dir.turn();
                pos = previous_pos;
                path.pop();
            }
            // we are outside, useful only for example input
            b'0' => {
                break;
            }
            _ => {
                visits[pos.0][pos.1] |= dir.to_bit();
            }
        }
        if let Some(new_pos) = dir.advance(pos) {
            previous_pos = pos;
            pos = new_pos;
            // check if guard has already been in this spot in the same direction
            if (visits[pos.0][pos.1] & dir.to_bit()) != 0 {
                return true;
            }
            path.push((pos, dir));
        } else {
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 41);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 5153);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1711);
    }
}
