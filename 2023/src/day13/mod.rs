use itertools::Itertools;

fn get_reflection_line(grid: Vec<&str>) -> u32 {
    let rows = grid
        .iter()
        .enumerate()
        .tuple_windows()
        .find(|((i, &a), (j, &b))| {
            // check it could be a reflection by iterating out
            for x in 0..=(grid.len() - j - 1).min(*i) {
                if grid[i - x] != grid[j + x] {
                    return false;
                }
            }

            true
        })
        .map_or(0, |((_, _), (rows, _))| rows);

    let columns = (0..grid[0].len())
        .tuple_windows()
        .find(|(i, j)| {
            for y in 0..=(grid[0].len() - j - 1).min(*i) {
                let column_i = grid.iter().map(|l| l.chars().nth(i - y).unwrap()).join("");
                let column_j = grid.iter().map(|l| l.chars().nth(j + y).unwrap()).join("");
                if column_i != column_j {
                    return false;
                }
            }
            true
        })
        .map_or(0, |(i, _)| i + 1);

    columns as u32 + rows as u32 * 100
}

fn parse_input(input: &str) -> (u32, u32) {
    let grids = input.trim().split("\n\n").map(|g| g.lines().collect_vec());

    let part1 = grids.map(get_reflection_line).sum();
    let part2 = 0;
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
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 405);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 27502);
        assert_eq!(part2, 0);
    }
}
