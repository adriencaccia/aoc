use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait NextDirection {
    fn next_from_direction(&self, direction: Direction) -> [((usize, usize), Direction); 3];
}

impl NextDirection for (usize, usize) {
    #[inline(always)]
    fn next_from_direction(&self, direction: Direction) -> [(Self, Direction); 3] {
        match direction {
            Direction::Up => [
                ((self.0 - 1, self.1), Direction::Up),
                ((self.0, self.1 - 1), Direction::Left),
                ((self.0, self.1 + 1), Direction::Right),
            ],
            Direction::Down => [
                ((self.0 + 1, self.1), Direction::Down),
                ((self.0, self.1 + 1), Direction::Right),
                ((self.0, self.1 - 1), Direction::Left),
            ],
            Direction::Left => [
                ((self.0, self.1 - 1), Direction::Left),
                ((self.0 - 1, self.1), Direction::Up),
                ((self.0 + 1, self.1), Direction::Down),
            ],
            Direction::Right => [
                ((self.0, self.1 + 1), Direction::Right),
                ((self.0 + 1, self.1), Direction::Down),
                ((self.0 - 1, self.1), Direction::Up),
            ],
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    direction: Direction,
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

const SIZE: usize = 141;

#[inline(always)]
fn parse(input: &str) -> ([[u8; SIZE]; SIZE], (usize, usize)) {
    let mut grid = [[b' '; SIZE]; SIZE];
    let mut start = (0, 0);

    input.lines().enumerate().for_each(|(i, l)| {
        l.bytes().enumerate().for_each(|(j, c)| {
            grid[i][j] = c;
            if c == b'S' {
                start = (i, j);
            }
        });
    });

    (grid, start)
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    let mut visited = [[false; SIZE]; SIZE];
    let mut p_queue = BinaryHeap::with_capacity(SIZE * SIZE);

    p_queue.push(State {
        cost: 0,
        x: start.0,
        y: start.1,
        direction: Direction::Right,
    });

    while let Some(cur) = p_queue.pop() {
        if grid[cur.x][cur.y] == b'E' {
            return cur.cost;
        }
        if visited[cur.x][cur.y] {
            continue;
        }
        visited[cur.x][cur.y] = true;

        for (next, direction) in (cur.x, cur.y).next_from_direction(cur.direction) {
            if grid[next.0][next.1] == b'#' {
                continue;
            }
            p_queue.push(State {
                cost: cur.cost + if direction == cur.direction { 1 } else { 1001 },
                x: next.0,
                y: next.1,
                direction,
            });
        }
    }

    unreachable!()
}

#[derive(Clone, Eq, PartialEq)]
struct StateWithPath {
    cost: usize,
    x: usize,
    y: usize,
    direction: Direction,
    path: Vec<(usize, usize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for StateWithPath {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs, making sure that the queue becomes a min-heap
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for StateWithPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse(input);
    let mut p_queue = BinaryHeap::with_capacity(SIZE * SIZE);

    p_queue.push(StateWithPath {
        cost: 0,
        x: start.0,
        y: start.1,
        direction: Direction::Right,
        path: vec![start],
    });

    let mut best_path_grid = grid;
    let mut min_cost = usize::MAX;
    let mut visited: HashMap<((usize, usize), Direction), usize> = HashMap::new();

    while let Some(cur) = p_queue.pop() {
        if grid[cur.x][cur.y] == b'E' {
            min_cost = min_cost.min(cur.cost);
            if cur.cost > min_cost {
                break;
            }
            cur.path.iter().for_each(|&(x, y)| {
                best_path_grid[x][y] = b'O';
            });
        }
        if let Some(&cost) = visited.get(&((cur.x, cur.y), cur.direction)) {
            if cur.cost > cost {
                continue;
            }
        } else {
            visited.insert(((cur.x, cur.y), cur.direction), cur.cost);
        }

        for (next, direction) in (cur.x, cur.y).next_from_direction(cur.direction) {
            if grid[next.0][next.1] == b'#' {
                continue;
            }
            let n_cost = cur.cost + if direction == cur.direction { 1 } else { 1001 };
            // avoid pushing useless state in the p_queue
            if let Some(&cost) = visited.get(&(next, direction)) {
                if n_cost >= cost {
                    continue;
                }
            }
            p_queue.push(StateWithPath {
                cost: n_cost,
                x: next.0,
                y: next.1,
                direction,
                path: cur
                    .path
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(next))
                    .collect(),
            });
        }
    }

    best_path_grid
        .iter()
        .flatten()
        .filter(|&&b| b == b'O')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 11048);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 123540);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 665);
    }
}
