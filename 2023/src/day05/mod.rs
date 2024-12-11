use std::ops::Range;

use itertools::Itertools;

fn parse_input(input: &str) -> (usize, usize) {
    let mut lines = input.trim().lines();

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
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
                    line.split_ascii_whitespace()
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

            for map in &maps {
                let last_seed = *seed_path.last().unwrap();
                let mut seed_in_ranges = false;
                for (destination_range_start, source_range_start, source_range_length) in map {
                    if (*source_range_start..(*source_range_start + *source_range_length))
                        .contains(&last_seed)
                    {
                        seed_path.push(destination_range_start + (last_seed - source_range_start));
                        seed_in_ranges = true;
                    }
                }
                if !seed_in_ranges {
                    seed_path.push(last_seed);
                }
            }

            seed_path
        })
        .collect_vec();

    let part1 = *seeds_paths
        .iter()
        .map(|paths| paths.last().unwrap())
        .min()
        .unwrap();

    // part 2
    let seeds_ranges: Vec<(usize, usize)> = seeds
        .chunks(2)
        .map(|chunk| chunk.iter().copied().collect_tuple().unwrap())
        .collect_vec();

    let mut overall_locations = vec![];

    for (start, length) in seeds_ranges {
        #[allow(clippy::single_range_in_vec_init)]
        let mut ranges: Vec<Range<usize>> = vec![start..(start + length)];

        let mut finished_ranges: Vec<Range<usize>> = Vec::new();
        for map in &maps {
            let mut new_ranges: Vec<Range<usize>> = Vec::new();
            for (destination_range_start, source_range_start, source_range_length) in map {
                for range in ranges.clone() {
                    let source_range_end = source_range_start + source_range_length;
                    if range.end <= *source_range_start || source_range_end <= range.start {
                        // unchanged range
                        new_ranges.push(range);
                    } else if range.start <= *source_range_start && source_range_end <= range.end {
                        // before
                        new_ranges.push(range.start..*source_range_start);
                        // middle
                        finished_ranges.push(
                            *destination_range_start
                                ..(destination_range_start + *source_range_length),
                        );
                        // after
                        new_ranges.push(source_range_end..range.end)
                    } else if range.start <= *source_range_start && range.end <= source_range_end {
                        // before
                        new_ranges.push(range.start..*source_range_start);
                        // middle
                        finished_ranges.push(
                            *destination_range_start
                                ..(destination_range_start + (range.end - *source_range_start)),
                        );
                    } else if *source_range_start <= range.start && range.end <= source_range_end {
                        // middle
                        finished_ranges.push(
                            (destination_range_start + (range.start - source_range_start))
                                ..((destination_range_start + (range.start - source_range_start))
                                    + (range.end - range.start)),
                        );
                    } else if *source_range_start <= range.start && range.start <= source_range_end
                    {
                        // middle
                        finished_ranges.push(
                            (destination_range_start + (range.start - source_range_start))
                                ..(destination_range_start
                                    + (range.start - source_range_start)
                                    + (source_range_end - range.start)),
                        );
                        // after
                        new_ranges.push(source_range_end..range.end);
                    }
                }
                ranges = new_ranges.clone();
                new_ranges.clear();
            }
            ranges.extend(finished_ranges.clone());
            finished_ranges.clear();
        }

        let locations = ranges
            .iter()
            .filter(|r| r.start != r.end)
            .map(|r| r.start)
            .collect_vec();
        overall_locations.extend(locations);
    }

    let part2 = *overall_locations.iter().min().unwrap();

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
        assert_eq!(part2, 46);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 289863851);
        assert_eq!(part2, 60568880);
    }
}
