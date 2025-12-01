#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
        let step_parity = if step.is_multiple_of(2) {
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

fn count_reachable_gardens(
    gardens: &mut Vec<Vec<Parity>>,
    start: (usize, usize),
    steps: usize,
    first_step_parity: Parity,
) -> usize {
    let mut parity = first_step_parity;

    gardens[start.0][start.1] = parity;

    for _step in 1..=steps {
        parity = parity.flip();
        for x in 0..gardens.len() {
            for y in 0..gardens[0].len() {
                let current_parity = gardens[x][y];
                if (current_parity == Parity::Even || current_parity == Parity::Odd)
                    && parity == current_parity.flip()
                {
                    if x > 0 && gardens[x - 1][y] == Parity::Empty {
                        gardens[x - 1][y] = parity;
                    }
                    if x < gardens.len() - 1 && gardens[x + 1][y] == Parity::Empty {
                        gardens[x + 1][y] = parity;
                    }
                    if y > 0 && gardens[x][y - 1] == Parity::Empty {
                        gardens[x][y - 1] = parity;
                    }
                    if y < gardens[0].len() - 1 && gardens[x][y + 1] == Parity::Empty {
                        gardens[x][y + 1] = parity;
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    print_visited_step(gardens, steps);

    gardens.iter().flatten().filter(|&p| *p == parity).count()
}

fn part1(input: &str, steps: usize) -> usize {
    let mut gardens: Vec<Vec<Parity>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Parity::Rock,
                    _ => Parity::Empty,
                })
                .collect()
        })
        .collect();

    let size = gardens.len();

    count_reachable_gardens(&mut gardens, (size / 2, size / 2), steps, Parity::Even)
}

fn part2(input: &str) -> usize {
    let gardens: Vec<Vec<Parity>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Parity::Rock,
                    _ => Parity::Empty,
                })
                .collect()
        })
        .collect();

    let size = gardens.len(); // 131
    assert!(size == gardens[0].len());
    let steps = 26501365; // 202300 * size + size / 2

    let big_grid_size = steps / size;
    assert!(big_grid_size == 202300);

    let mut total = 0;
    let center = (size / 2, size / 2);

    let blue_grids = big_grid_size.pow(2);
    let blue_grid_count =
        count_reachable_gardens(&mut gardens.clone(), center, size + 1, Parity::Odd);
    total += blue_grid_count * blue_grids;

    let red_grids = (big_grid_size - 1).pow(2);
    let red_grid_count = count_reachable_gardens(&mut gardens.clone(), center, size, Parity::Even);
    total += red_grid_count * red_grids;

    // big grid top
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size - 1, size / 2),
        size - 1,
        Parity::Odd,
    );
    // big grid bottom
    total += count_reachable_gardens(&mut gardens.clone(), (0, size / 2), size - 1, Parity::Odd);
    // big grid left
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size / 2, size - 1),
        size - 1,
        Parity::Odd,
    );
    // big grid right
    total += count_reachable_gardens(&mut gardens.clone(), (size / 2, 0), size - 1, Parity::Odd);

    // bottom left
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size - 1, 0),
        size / 2 - 1,
        Parity::Odd,
    ) * big_grid_size;
    // bottom right
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size - 1, size - 1),
        size / 2 - 1,
        Parity::Odd,
    ) * big_grid_size;
    // top left
    total += count_reachable_gardens(&mut gardens.clone(), (0, 0), size / 2 - 1, Parity::Odd)
        * big_grid_size;
    // top right
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (0, size - 1),
        size / 2 - 1,
        Parity::Odd,
    ) * big_grid_size;

    // inside bottom left
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size - 1, 0),
        3 * size / 2 - 1,
        Parity::Even,
    ) * (big_grid_size - 1);
    // inside bottom right
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (size - 1, size - 1),
        3 * size / 2 - 1,
        Parity::Even,
    ) * (big_grid_size - 1);
    // inside top left
    total += count_reachable_gardens(&mut gardens.clone(), (0, 0), 3 * size / 2 - 1, Parity::Even)
        * (big_grid_size - 1);
    // inside top right
    total += count_reachable_gardens(
        &mut gardens.clone(),
        (0, size - 1),
        3 * size / 2 - 1,
        Parity::Even,
    ) * (big_grid_size - 1);

    total
}

pub fn main() -> (usize, usize) {
    let part1 = part1(include_str!("input.txt"), 64);
    let part2 = part2(include_str!("input.txt"));
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
        assert_eq!(part2, 584211423220706);
    }
}
