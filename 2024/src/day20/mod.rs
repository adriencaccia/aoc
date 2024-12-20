const SIZE: usize = 141;

#[derive(Debug, Clone, Copy)]
struct Item {
    pos: (usize, usize),
    distance_to_end: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const CHEATS: [(i32, i32); 4] = [(2, 0), (-2, 0), (0, 2), (0, -2)];

fn parse(input: &str) -> ([[u8; 141]; 141], Vec<Item>, [[usize; 141]; 141]) {
    let mut grid = [[b'#'; SIZE]; SIZE];
    let mut start = (0, 0);
    let mut end = (0, 0);

    input.lines().enumerate().for_each(|(i, line)| {
        line.as_bytes().iter().enumerate().for_each(|(j, &b)| {
            grid[i][j] = b;
            if b == b'S' {
                start = (i, j);
            } else if b == b'E' {
                end = (i, j);
            }
        });
    });

    let mut path_items = Vec::with_capacity(SIZE * SIZE);
    let mut stack = Vec::with_capacity(SIZE * SIZE);
    stack.push(start);

    while let Some(pos) = stack.pop() {
        path_items.push(Item {
            pos,
            distance_to_end: 0,
        });
        if pos == end {
            break;
        }
        let previous_visited = if path_items.len() > 1 {
            path_items[path_items.len() - 2].pos
        } else {
            (0, 0) // Dummy value to handle the first iteration
        };

        for (dx, dy) in &DIRECTIONS {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx < 0 || ny < 0 || nx >= SIZE as i32 || ny >= SIZE as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if grid[nx][ny] == b'#' {
                continue;
            }
            if (nx, ny) == previous_visited {
                continue;
            }
            stack.push((nx, ny));
        }
    }

    let path_length = path_items.len();
    for (i, item) in path_items.iter_mut().enumerate() {
        item.distance_to_end = path_length - i - 1;
    }

    let mut d_grid = [[usize::MAX; SIZE]; SIZE];
    for item in &path_items {
        d_grid[item.pos.0][item.pos.1] = item.distance_to_end;
    }
    (grid, path_items, d_grid)
}

pub fn part1(input: &str) -> u32 {
    let (grid, path_items, d_grid) = parse(input);

    let mut valid_cheats = 0;

    for item in &path_items {
        for (dx, dy) in &CHEATS {
            let nx = item.pos.0 as i32 + dx;
            let ny = item.pos.1 as i32 + dy;
            if nx < 0
                || ny < 0
                || nx >= SIZE as i32
                || ny >= SIZE as i32
                || grid[nx as usize][ny as usize] == b'#'
            {
                continue;
            }
            let nx1 = item.pos.0 as i32 + dx / 2;
            let ny1 = item.pos.1 as i32 + dy / 2;
            if grid[nx1 as usize][ny1 as usize] != b'#' {
                continue;
            }
            let distance_gained =
                d_grid[item.pos.0][item.pos.1] as i32 - 2 - d_grid[nx as usize][ny as usize] as i32;
            if distance_gained >= 100 {
                valid_cheats += 1;
            }
        }
    }
    valid_cheats
}

pub fn part2(input: &str) -> u32 {
    let (grid, path_items, d_grid) = parse(input);

    let mut valid_cheats = 0;

    for item in &path_items {
        for dx in -20i32..=20 {
            for dy in -20i32..=20 {
                if dx.abs() + dy.abs() > 20 {
                    continue;
                }
                let nx = item.pos.0 as i32 + dx;
                let ny = item.pos.1 as i32 + dy;
                if nx < 0
                    || ny < 0
                    || nx >= SIZE as i32
                    || ny >= SIZE as i32
                    || grid[nx as usize][ny as usize] == b'#'
                {
                    continue;
                }
                let distance_gained = d_grid[item.pos.0][item.pos.1] as i32
                    - (dx.abs() + dy.abs())
                    - d_grid[nx as usize][ny as usize] as i32;
                if distance_gained >= 100 {
                    valid_cheats += 1;
                }
            }
        }
    }

    valid_cheats
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1511);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 1020507);
    }
}
