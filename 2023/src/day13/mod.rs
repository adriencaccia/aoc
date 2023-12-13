use itertools::Itertools;

fn get_reflection_line(grid: Vec<&str>) -> u32 {
    let rows = (0..grid.len())
        .tuple_windows()
        .find(|(i, j)| {
            // check it could be a reflection by iterating out
            for x in 0..=(grid.len() - j - 1).min(*i) {
                if grid[i - x] != grid[j + x] {
                    return false;
                }
            }

            true
        })
        .map_or(0, |(_, j)| j as u32 * 100);

    if rows != 0 {
        return rows;
    }

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
        .map_or(0, |(_, j)| j);

    columns as u32
}

fn get_reflection_line_with_smudge(grid: Vec<&str>) -> u32 {
    let rows = (0..grid.len())
        .tuple_windows()
        .find(|(i, j)| {
            let mut smudge_used = false;
            // check it could be a reflection by iterating out
            for x in 0..=(grid.len() - j - 1).min(*i) {
                for idx in 0..grid[0].len() {
                    let char_a = grid[i - x].chars().nth(idx).unwrap();
                    let char_b = grid[j + x].chars().nth(idx).unwrap();
                    if !smudge_used && char_a != char_b {
                        smudge_used = true;
                        continue;
                    }
                    if char_a != char_b {
                        return false;
                    }
                }
            }
            smudge_used
        })
        .map_or(0, |(_, j)| j as u32 * 100);

    if rows != 0 {
        return rows;
    }

    let columns = (0..grid[0].len())
        .tuple_windows()
        .find(|(i, j)| {
            let mut smudge_used = false;
            for y in 0..=(grid[0].len() - j - 1).min(*i) {
                let column_i = grid.iter().map(|l| l.chars().nth(i - y).unwrap()).join("");
                let column_j = grid.iter().map(|l| l.chars().nth(j + y).unwrap()).join("");
                for idx in 0..grid.len() {
                    let char_a = column_i.chars().nth(idx).unwrap();
                    let char_b = column_j.chars().nth(idx).unwrap();
                    if !smudge_used && char_a != char_b {
                        smudge_used = true;
                        continue;
                    }
                    if char_a != char_b {
                        return false;
                    }
                }
            }
            smudge_used
        })
        .map_or(0, |(_, j)| j);

    columns as u32
}

fn parse_input(input: &str) -> (u32, u32) {
    let grids = input.trim().split("\n\n").map(|g| g.lines().collect_vec());

    let part1 = grids.clone().map(get_reflection_line).sum();
    let part2 = grids.map(get_reflection_line_with_smudge).sum();
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
        assert_eq!(part2, 400);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 27502);
        assert_eq!(part2, 31947);
    }
}
