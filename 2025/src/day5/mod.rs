pub fn part1(input: &str) -> u32 {
    let (ranges_str, ids_str) = input.trim_ascii().split_once("\n\n").unwrap();

    let ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ids: Vec<usize> = ids_str.lines().map(|line| line.parse().unwrap()).collect();

    ids.iter().fold(0, |acc, &id| {
        if ranges.iter().any(|&(start, end)| id >= start && id <= end) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part2(input: &str) -> usize {
    let ranges_str = input.trim_ascii().split("\n\n").next().unwrap();

    let mut ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    // Merge overlapping ranges
    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();
    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
        .iter()
        .fold(0, |acc, &(start, end)| acc + (end - start + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 840);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 359913027576322);
    }
}
