use itertools::Itertools;

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.trim().lines();

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect_vec();

    let maps: Vec<Vec<(usize, usize, usize)>> = lines
        .skip(1)
        .collect_vec()
        .split(|line| line.is_empty())
        .map(|map| {
            map.get(1..map.len())
                .unwrap()
                .iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let seeds_paths: Vec<Vec<usize>> = seeds
        .iter()
        .map(|seed| {
            let mut seed_path: Vec<usize> = vec![*seed];

            maps.iter().for_each(|map| {
                let last_seed = *seed_path.last().unwrap();
                let mut seed_in_ranges = false;
                for (destination_range_start, source_range_start, range_length) in map {
                    if (*source_range_start..(*source_range_start + *range_length))
                        .contains(&last_seed)
                    {
                        seed_path
                            .push(*destination_range_start + (last_seed - *source_range_start));
                        seed_in_ranges = true;
                    }
                }
                if !seed_in_ranges {
                    seed_path.push(last_seed);
                }
            });

            seed_path
        })
        .collect_vec();

    let part1 = *seeds_paths
        .iter()
        .map(|paths| paths.last().unwrap())
        .min()
        .unwrap();
    let part2 = 0;
    (part1, part2)
}

pub fn main() -> (usize, usize) {
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
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 35);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 289863851);
        assert_eq!(part2, 0);
    }
}
