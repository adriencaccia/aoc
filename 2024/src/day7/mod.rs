use itertools::Itertools;

const SIZE: usize = 12;

#[inline(always)]
fn is_solvable(total: u64, acc: u64, values: &[u64]) -> bool {
    if acc > total {
        return false;
    }
    if values.len() == 1 {
        return acc + values[0] == total || acc * values[0] == total;
    }

    is_solvable(total, acc + values[0], &values[1..])
        || is_solvable(total, acc * values[0], &values[1..])
}

pub fn part1(input: &str) -> u64 {
    input.lines().fold(0, |sum, l| {
        let (total, values_str) = l.split(": ").collect_tuple().unwrap();
        let total: u64 = total.parse().unwrap();
        let mut values: Vec<u64> = Vec::with_capacity(SIZE);
        values_str
            .split_whitespace()
            .for_each(|v| values.push(v.parse().unwrap()));

        if is_solvable(total, values[0], &values[1..]) {
            return sum + total;
        }
        sum
    })
}

#[inline(always)]
fn is_solvable_2(total: u64, acc: u64, values: &[u64]) -> bool {
    if acc > total {
        return false;
    }
    if values.len() == 1 {
        return acc + values[0] == total
            || acc * values[0] == total
            || acc * 10_u64.pow((values[0] as f64).log10().floor() as u32 + 1) + values[0]
                == total;
    }

    is_solvable_2(total, acc + values[0], &values[1..])
        || is_solvable_2(total, acc * values[0], &values[1..])
        || is_solvable_2(
            total,
            acc * 10_u64.pow((values[0] as f64).log10().floor() as u32 + 1) + values[0],
            &values[1..],
        )
}

pub fn part2(input: &str) -> u64 {
    input.lines().fold(0, |sum, l| {
        let (total, values_str) = l.split(": ").collect_tuple().unwrap();
        let total: u64 = total.parse().unwrap();
        let mut values: Vec<u64> = Vec::with_capacity(SIZE);
        values_str
            .split_whitespace()
            .for_each(|v| values.push(v.parse().unwrap()));

        if is_solvable_2(total, values[0], &values[1..]) {
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
