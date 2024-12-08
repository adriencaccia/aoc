use itertools::Itertools;

const SIZE: usize = 12;

/// Check if target can be obtained by looking at the last value of `values`,
/// to trim the possible solutions more efficiently
#[inline(always)]
fn is_solvable(target: u64, values: &[u64]) -> bool {
    let len = values.len();
    if len == 1 {
        return target == values[0];
    }
    let last = values[len - 1];
    if target % last == 0 && is_solvable(target / last, &values[..(len - 1)]) {
        return true;
    }
    if target > last && is_solvable(target - last, &values[..(len - 1)]) {
        return true;
    }

    false
}

pub fn part1(input: &str) -> u64 {
    input.lines().fold(0, |sum, l| {
        let (total, values_str) = l.split(": ").collect_tuple().unwrap();
        let total: u64 = total.parse().unwrap();
        let mut values: Vec<u64> = Vec::with_capacity(SIZE);
        values_str
            .split_whitespace()
            .for_each(|v| values.push(v.parse().unwrap()));

        if is_solvable(total, &values) {
            return sum + total;
        }
        sum
    })
}

#[inline(always)]
fn is_solvable_2(target: u64, values: &[u64]) -> bool {
    let len = values.len();
    if len == 1 {
        return target == values[0];
    }
    let last = values[len - 1];
    if target % last == 0 && is_solvable_2(target / last, &values[..(len - 1)]) {
        return true;
    }
    if target > last && is_solvable_2(target - last, &values[..(len - 1)]) {
        return true;
    }
    let last_length = 10_u64.pow((last as f64).log10().floor() as u32 + 1);
    if target % last_length == last && is_solvable_2(target / last_length, &values[..(len - 1)]) {
        return true;
    }

    false
}

pub fn part2(input: &str) -> u64 {
    input.lines().fold(0, |sum, l| {
        let (total, values_str) = l.split(": ").collect_tuple().unwrap();
        let total: u64 = total.parse().unwrap();
        let mut values: Vec<u64> = Vec::with_capacity(SIZE);
        values_str
            .split_whitespace()
            .for_each(|v| values.push(v.parse().unwrap()));

        if is_solvable_2(total, &values) {
            return sum + total;
        }
        sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1298103531759);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 11387);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 140575048428831);
    }
}
