use std::collections::HashMap;

use itertools::Itertools;

fn count_configs<'a, 'b>(
    springs: &'a str,
    groups: &'b [usize],
    solves: &mut HashMap<(&'a str, &'b [usize]), u64>,
) -> u64 {
    if springs.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }
    if groups.is_empty() {
        return if springs.contains('#') { 0 } else { 1 };
    }

    if let Some(&cached) = solves.get(&(springs, groups)) {
        return cached;
    }

    let first_char = springs.chars().next().unwrap();

    let mut count = 0;
    // consider first_char as a operational spring
    if ".?".contains(first_char) {
        count += count_configs(&springs[1..], groups, solves);
    }

    // consider first_char as a damaged spring
    if "#?".contains(first_char) {
        // if the first group fits, ie there are no operational spring
        if groups[0] <= springs.len()
            && !springs[..groups[0]].contains('.')
            && (groups[0] == springs.len() || springs.chars().nth(groups[0]).unwrap() != '#')
        {
            count += if groups[0] == springs.len() {
                count_configs("", &groups[1..], solves)
            } else {
                count_configs(&springs[groups[0] + 1..], &groups[1..], solves)
            };
        }
    }

    solves.insert((springs, groups), count);
    count
}

fn parse_input(input: &str) -> (u64, u64) {
    let lines = input.trim().lines().map(|line| {
        let (springs, groups) = line.split(' ').collect_tuple().unwrap();
        (
            springs,
            groups
                .split(',')
                .map(|g| g.parse::<usize>().unwrap())
                .collect_vec(),
        )
    });

    let part1 = lines
        .clone()
        .map(|(springs, groups)| {
            let mut solves: HashMap<(&str, &[usize]), u64> = HashMap::new();
            count_configs(springs, groups.as_slice(), &mut solves)
        })
        .sum();

    let part2 = lines
        .map(|(springs, groups)| {
            let unfolded_springs = (0..5).map(|_| springs).join("?");
            let unfolded_groups = vec![groups; 5].concat();
            let mut solves: HashMap<(&str, &[usize]), u64> = HashMap::new();
            count_configs(
                unfolded_springs.as_str(),
                unfolded_groups.as_slice(),
                &mut solves,
            )
        })
        .sum();
    (part1, part2)
}

pub fn main() -> (u64, u64) {
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
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 21);
        assert_eq!(part2, 525152);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 6949);
        assert_eq!(part2, 51456609952403);
    }
}
