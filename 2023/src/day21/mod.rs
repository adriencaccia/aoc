#[derive(PartialEq, Debug, Clone, Copy)]
enum Parity {
    Even,
    Odd,
    Empty,
    Rock,
}

impl Parity {
    fn flip(&self) -> Self {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
            _ => *self,
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
            (Parity::Even, Parity::Even) => "🟪",
            (Parity::Odd, Parity::Odd) => "🟪",
            (Parity::Rock, _) => "⬜",
            _ => "⬛",
        }
    }
}

#[cfg(debug_assertions)]
fn print_visited_step(visited: &Vec<Vec<Parity>>, step: usize) {
    use itertools::Itertools;

    println!("Step {}", step);
    for row in visited {
        println!("{}", row.iter().map(|r| r.to_symbol(step)).join(""));
    }
}

fn part1(input: &str, steps: usize) -> u32 {
    let mut visited_gardens: Vec<Vec<Parity>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'S' => Parity::Even,
                    '#' => Parity::Rock,
                    _ => Parity::Empty,
                })
                .collect()
        })
        .collect();

    for step in 1..=steps {
        let step_parity = if step % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        };

        for x in 0..visited_gardens.len() {
            for y in 0..visited_gardens[0].len() {
                let current_parity = visited_gardens[x][y];
                if (current_parity == Parity::Even || current_parity == Parity::Odd)
                    && step_parity == current_parity.flip()
                {
                    if x > 0 && visited_gardens[x - 1][y] == Parity::Empty {
                        visited_gardens[x - 1][y] = step_parity;
                    }
                    if x < visited_gardens.len() - 1 && visited_gardens[x + 1][y] == Parity::Empty {
                        visited_gardens[x + 1][y] = step_parity;
                    }
                    if y > 0 && visited_gardens[x][y - 1] == Parity::Empty {
                        visited_gardens[x][y - 1] = step_parity;
                    }
                    if y < visited_gardens[0].len() - 1
                        && visited_gardens[x][y + 1] == Parity::Empty
                    {
                        visited_gardens[x][y + 1] = step_parity;
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    print_visited_step(&visited_gardens, 64);

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
