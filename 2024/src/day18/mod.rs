use std::{cmp::Ordering, collections::BinaryHeap};

use itertools::Itertools;

// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// impl Direction {
//     #[inline(always)]
//     fn dx_dy(&self) -> (i32, i32) {
//         match self {
//             Direction::Up => (-1, 0),
//             Direction::Down => (1, 0),
//             Direction::Left => (0, -1),
//             Direction::Right => (0, 1),
//         }
//     }
// }

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs, making sure that the queue becomes a min-heap
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const SIZE: usize = 71;
const BYTES_SIZE: usize = 4000;

#[inline(always)]
fn parse(input: &str) -> ([[u8; SIZE]; SIZE], Vec<(usize, usize)>) {
    let grid = [[b'.'; SIZE]; SIZE];
    let mut bytes: Vec<(usize, usize)> = Vec::with_capacity(BYTES_SIZE);
    bytes.extend(input.lines().map(|l| {
        l.split(",")
            .map(|c| c.parse().unwrap())
            .collect_tuple::<(usize, usize)>()
            .unwrap()
    }));

    (grid, bytes)
}

#[inline(always)]
fn find_path(grid: &[[u8; SIZE]; SIZE]) -> Option<usize> {
    let mut p_queue = BinaryHeap::with_capacity(SIZE * SIZE);
    let mut visited = [[false; SIZE]; SIZE];
    p_queue.push(State {
        cost: 0,
        x: 0,
        y: 0,
    });

    while let Some(cur) = p_queue.pop() {
        if cur.x == SIZE - 1 && cur.y == SIZE - 1 {
            return Some(cur.cost);
        }
        if visited[cur.x][cur.y] {
            continue;
        }
        visited[cur.x][cur.y] = true;

        for (dx, dy) in &DIRECTIONS {
            let nx = cur.x as i32 + dx;
            let ny = cur.y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= SIZE as i32 || ny >= SIZE as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if visited[nx][ny] {
                continue;
            }
            if grid[nx][ny] == b'#' {
                continue;
            }
            p_queue.push(State {
                cost: cur.cost + 1,
                x: nx,
                y: ny,
            });
        }
    }

    None
}

pub fn part1(input: &str) -> u32 {
    let (mut grid, bytes) = parse(input);

    for (x, y) in &bytes[..1024] {
        grid[*x][*y] = b'#';
    }

    find_path(&grid).unwrap() as u32
}

pub fn part2(input: &str) -> String {
    let (grid, bytes) = parse(input);

    let mut left = 0;
    let mut right = bytes.len();

    while left < right {
        let mid = (left + right) / 2;
        let mut grid = grid;

        for (x, y) in &bytes[..mid] {
            grid[*x][*y] = b'#';
        }

        if find_path(&grid).is_none() {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    let (x, y) = bytes[left - 1];
    format!("{},{}", x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
"};

    #[test]
    #[ignore]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 22);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 348);
    }

    #[test]
    #[ignore]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), "6,1");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), "54,44");
    }
}
