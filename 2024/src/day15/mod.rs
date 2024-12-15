use arrayvec::ArrayVec;

const SIZE: usize = 50;
const INSTRUCTIONS_SIZE: usize = 20_000;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[inline(always)]
    fn from_byte(b: u8) -> Self {
        match b {
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn dx_dy(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

fn move_tile<const N: usize>(
    grid: &mut [[u8; SIZE * N]; SIZE],
    pos: (usize, usize),
    (dx, dy): (i32, i32),
) -> (usize, usize) {
    let (nx, ny) = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);

    if grid[nx][ny] == b'#' {
        return pos;
    }
    if grid[nx][ny] == b'.' {
        grid[nx][ny] = grid[pos.0][pos.1];
        grid[pos.0][pos.1] = b'.';
        return (nx, ny);
    }

    move_tile(grid, (nx, ny), (dx, dy));
    if grid[nx][ny] == b'.' {
        grid[nx][ny] = grid[pos.0][pos.1];
        grid[pos.0][pos.1] = b'.';
        return (nx, ny);
    }
    pos
}

#[inline(always)]
fn compute_gps<const N: usize>(grid: &[[u8; SIZE * N]; SIZE]) -> u32 {
    grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, &c)| {
            if N == 1 && c == b'O' || N == 2 && c == b'[' {
                acc + i * 100 + j
            } else {
                acc
            }
        }) as u32
    })
}

#[inline(always)]
fn parse_instructions(input: &str) -> ArrayVec<Direction, INSTRUCTIONS_SIZE> {
    input
        .bytes()
        .filter(|&b| b != b'\n')
        .map(Direction::from_byte)
        .collect()
}

#[inline(always)]
fn parse(
    input: &str,
) -> (
    [[u8; SIZE]; SIZE],
    (usize, usize),
    ArrayVec<Direction, INSTRUCTIONS_SIZE>,
) {
    let mut grid = [[b' '; SIZE]; SIZE];
    let mut robot_pos = (0, 0);

    let mut it = input.split("\n\n");
    it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'@' {
                robot_pos = (i, j);
            }
        });
    });

    let instructions = parse_instructions(it.next().unwrap());

    (grid, robot_pos, instructions)
}

pub fn part1(input: &str) -> u32 {
    let (mut grid, mut robot_pos, instructions) = parse(input);

    for dir in instructions {
        robot_pos = move_tile::<1>(&mut grid, robot_pos, dir.dx_dy());
    }

    compute_gps::<1>(&grid)
}

#[inline(always)]
fn parse_2(
    input: &str,
) -> (
    [[u8; SIZE * 2]; SIZE],
    (usize, usize),
    ArrayVec<Direction, INSTRUCTIONS_SIZE>,
) {
    let mut grid = [[b' '; SIZE * 2]; SIZE];
    let mut robot_pos = (0, 0);

    let mut it = input.split("\n\n");
    it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, c)| match c {
            b'#' => {
                grid[i][2 * j] = b'#';
                grid[i][2 * j + 1] = b'#';
            }
            b'O' => {
                grid[i][2 * j] = b'[';
                grid[i][2 * j + 1] = b']';
            }
            b'.' => {
                grid[i][2 * j] = b'.';
                grid[i][2 * j + 1] = b'.';
            }
            b'@' => {
                robot_pos = (i, 2 * j);
                grid[i][2 * j] = b'@';
                grid[i][2 * j + 1] = b'.';
            }
            _ => unreachable!(),
        });
    });

    let instructions = parse_instructions(it.next().unwrap());

    (grid, robot_pos, instructions)
}

fn can_move_up_or_down(
    grid: &mut [[u8; SIZE * 2]; SIZE],
    pos: (usize, usize),
    (dx, dy): (i32, i32),
) -> bool {
    let nx = (pos.0 as i32 + dx) as usize;
    let ny0 = pos.1 - 1;
    let ny1 = pos.1;

    // handle robot case separately
    if grid[pos.0][pos.1] == b'@' {
        match grid[nx][ny1] {
            b'#' => return false,
            b'.' => return true,
            b']' => return can_move_up_or_down(grid, (nx, ny0), (dx, dy)),
            b'[' => return can_move_up_or_down(grid, (nx, ny1), (dx, dy)),
            _ => unreachable!(),
        }
    }

    // handle box case
    let ny2 = pos.1 + 1;
    let c1 = grid[nx][ny1];
    let c2 = grid[nx][ny2];

    match (c1, c2) {
        (b'#', _) => false,
        (_, b'#') => false,
        (b'.', b'.') => true,
        (b']', b'.') => can_move_up_or_down(grid, (nx, ny0), (dx, dy)),
        (b'[', b']') => can_move_up_or_down(grid, (nx, ny1), (dx, dy)),
        (b'.', b'[') => can_move_up_or_down(grid, (nx, ny2), (dx, dy)),
        (b']', b'[') => {
            can_move_up_or_down(grid, (nx, ny0), (dx, dy))
                && can_move_up_or_down(grid, (nx, ny2), (dx, dy))
        }
        _ => unreachable!(),
    }
}

fn move_up_or_down(
    grid: &mut [[u8; SIZE * 2]; SIZE],
    pos: (usize, usize),
    (dx, dy): (i32, i32),
) -> (usize, usize) {
    let nx = (pos.0 as i32 + dx) as usize;
    let ny1 = pos.1;

    if grid[nx][ny1] == b'#' {
        return pos;
    }
    // handle robot case separately
    let ny0 = pos.1 - 1;
    let ny2 = pos.1 + 1;
    if grid[pos.0][pos.1] == b'@' {
        match grid[nx][ny1] {
            b']' => {
                move_up_or_down(grid, (nx, ny0), (dx, dy));
                grid[nx][ny0] = b'.';
            }
            b'[' => {
                move_up_or_down(grid, (nx, ny1), (dx, dy));
                grid[nx][ny2] = b'.';
            }
            _ => {}
        };
        grid[nx][ny1] = grid[pos.0][pos.1];
        grid[pos.0][pos.1] = b'.';
        return (nx, ny1);
    }

    // handle box case
    let c1 = grid[nx][ny1];
    let c2 = grid[nx][ny2];

    match (c1, c2) {
        (b'.', b'[') => {
            move_up_or_down(grid, (nx, ny2), (dx, dy));
        }
        (b'[', b']') => {
            move_up_or_down(grid, (nx, ny1), (dx, dy));
        }
        (b']', b'.') => {
            move_up_or_down(grid, (nx, ny0), (dx, dy));
        }
        (b']', b'[') => {
            move_up_or_down(grid, (nx, ny0), (dx, dy));
            move_up_or_down(grid, (nx, ny2), (dx, dy));
        }
        _ => {}
    };
    grid[nx][ny1] = grid[pos.0][pos.1];
    grid[nx][ny2] = grid[pos.0][pos.1 + 1];
    grid[pos.0][pos.1] = b'.';
    grid[pos.0][pos.1 + 1] = b'.';
    (nx, ny1)
}

pub fn part2(input: &str) -> u32 {
    let (mut grid, mut robot_pos, instructions) = parse_2(input);

    for dir in instructions {
        match dir {
            Direction::Left | Direction::Right => {
                robot_pos = move_tile::<2>(&mut grid, robot_pos, dir.dx_dy());
            }
            Direction::Down | Direction::Up
                if can_move_up_or_down(&mut grid, robot_pos, dir.dx_dy()) =>
            {
                robot_pos = move_up_or_down(&mut grid, robot_pos, dir.dx_dy());
            }
            _ => {}
        }
    }

    compute_gps::<2>(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 10092);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1406628);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 9021);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1432781);
    }
}
