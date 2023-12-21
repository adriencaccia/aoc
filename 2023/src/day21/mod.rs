use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
enum Tile {
    #[display(".")]
    Garden,
    #[display("#")]
    Rock,
    #[display("S")]
    Start,
    Possible,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Parity {
    Even,
    Odd,
    Empty,
}

impl Parity {
    fn flip(&self) -> Self {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
            Parity::Empty => Parity::Empty,
        }
    }

    #[cfg(debug_assertions)]
    fn to_symbol(self, step: usize) -> &'static str {
        let step_parity = if step % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        };
        match (&self, step_parity) {
            (Parity::Even, Parity::Even) => "O",
            (Parity::Odd, Parity::Odd) => "O",
            _ => ".",
        }
    }
}

#[cfg(debug_assertions)]
fn print_visited_step(visited: &Vec<Vec<Parity>>, step: usize) {
    println!("Step {}", step);
    for row in visited {
        for parity in row {
            print!("{}", parity.to_symbol(step));
        }
        println!();
    }
    println!();
}

fn part1(input: &str, steps: usize) -> u32 {
    let grid: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let starting_position = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if *tile == Tile::Start {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut visited_gardens: Vec<Vec<Parity>> =
        vec![vec![Parity::Empty; grid[0].len()]; grid.len()];
    visited_gardens[starting_position.1][starting_position.0] = Parity::Even;

    for step in 1..=steps {
        let step_parity = if step % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        };

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                match visited_gardens[x][y] {
                    Parity::Even | Parity::Odd if step_parity == visited_gardens[x][y].flip() => {
                        if x > 0
                            && grid[x - 1][y] == Tile::Garden
                            && visited_gardens[x - 1][y] == Parity::Empty
                        {
                            visited_gardens[x - 1][y] = step_parity;
                        }
                        if x < grid.len() - 1
                            && grid[x + 1][y] == Tile::Garden
                            && visited_gardens[x + 1][y] == Parity::Empty
                        {
                            visited_gardens[x + 1][y] = step_parity;
                        }
                        if y > 0
                            && grid[x][y - 1] == Tile::Garden
                            && visited_gardens[x][y - 1] == Parity::Empty
                        {
                            visited_gardens[x][y - 1] = step_parity;
                        }
                        if y < grid[0].len() - 1
                            && grid[x][y + 1] == Tile::Garden
                            && visited_gardens[x][y + 1] == Parity::Empty
                        {
                            visited_gardens[x][y + 1] = step_parity;
                        }
                    }
                    _ => continue,
                }
            }
        }
        #[cfg(debug_assertions)]
        print_visited_step(&visited_gardens, step);
    }

    visited_gardens
        .into_iter()
        .flatten()
        .filter(|&p| p == Parity::Even)
        .count() as u32
}

pub fn main() -> (u32, u32) {
    let part1 = part1(include_str!("input.txt"), 64);
    let part2 = 0;
    println!("part1 {}", part1);
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const EXAMPLE_INPUT: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    #[test]
    fn test_example_part1() {
        let part1 = part1(EXAMPLE_INPUT, 6);

        assert_eq!(part1, 16);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 3503);
        assert_eq!(part2, 0);
    }
}
