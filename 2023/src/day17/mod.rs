use itertools::Itertools;
use std::{cmp::Ordering, collections::BinaryHeap};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, EnumIter)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn to_usize(self) -> usize {
        match self {
            Direction::North => 0,
            Direction::West => 1,
            Direction::South => 2,
            Direction::East => 3,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
    direction: Option<Direction>,
    same_direction_moves: usize,
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

/// Based on https://doc.rust-lang.org/std/collections/binary_heap/index.html.
///
/// Do not use the dist matrix, as the shortest path to the very end is not necessarily including the shortest path to every of its nodes.
/// Because of the maximum 3 moves in the same direction rule.
pub fn shortest_path(
    grid: &Vec<Vec<u32>>,
    min_same_direction: usize,
    max_same_direction: usize,
) -> u32 {
    // position, direction, times moved same direction
    let mut visited: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; max_same_direction + 1]; 4]; grid.len() * grid[0].len()];

    let mut heap = BinaryHeap::new();

    // top left
    let start = 0;
    // bottom right
    let goal = (grid.len() - 1) * grid.len() + (grid[0].len() - 1);

    heap.push(State {
        cost: 0,
        position: start,
        direction: None,
        same_direction_moves: 0,
    });

    while let Some(State {
        cost,
        position,
        direction,
        same_direction_moves,
    }) = heap.pop()
    {
        let x = position / grid.len();
        let y = position % grid.len();

        if position == goal && same_direction_moves >= min_same_direction {
            return cost as u32;
        }

        // prevent loops by checking if state already reached
        if direction.is_some()
            && visited[position][direction.unwrap().to_usize()][same_direction_moves]
        {
            continue;
        }
        if direction.is_some() {
            visited[position][direction.unwrap().to_usize()][same_direction_moves] = true;
        }

        for next_direction in Direction::iter() {
            let new_x = match next_direction {
                Direction::North => {
                    if x == 0 {
                        continue;
                    }
                    x - 1
                }
                Direction::South => {
                    if x == grid.len() - 1 {
                        continue;
                    }
                    x + 1
                }
                _ => x,
            };
            let new_y = match next_direction {
                Direction::West => {
                    if y == 0 {
                        continue;
                    }
                    y - 1
                }
                Direction::East => {
                    if y == grid[0].len() - 1 {
                        continue;
                    }
                    y + 1
                }
                _ => y,
            };

            if Some(next_direction) == direction && same_direction_moves < max_same_direction {
                let next = State {
                    cost: cost + grid[new_x][new_y] as usize,
                    position: new_x * grid.len() + new_y,
                    direction,
                    same_direction_moves: same_direction_moves + 1,
                };
                heap.push(next);
            }
            if direction.is_some() && same_direction_moves < min_same_direction {
                continue;
            }
            match (direction, next_direction) {
                (Some(Direction::North), Direction::South | Direction::North) => continue,
                (Some(Direction::East), Direction::West | Direction::East) => continue,
                (Some(Direction::South), Direction::North | Direction::South) => continue,
                (Some(Direction::West), Direction::East | Direction::West) => continue,
                _ => (),
            };

            let next = State {
                cost: cost + grid[new_x][new_y] as usize,
                position: new_x * grid.len() + new_y,
                direction: Some(next_direction),
                same_direction_moves: 1,
            };
            heap.push(next);
        }
    }

    // Goal not reachable
    unreachable!()
}

fn parse_input(input: &str) -> (u32, u32) {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let part1 = shortest_path(&grid, 1, 3);
    let part2 = shortest_path(&grid, 4, 10);
    (part1, part2)
}

pub fn main() -> (u32, u32) {
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
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 102);
        assert_eq!(part2, 94);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 635);
        assert_eq!(part2, 734);
    }
}
