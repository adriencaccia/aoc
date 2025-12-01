use rustc_hash::FxHashMap;

#[inline(always)]
fn get_num_digits(v: u64) -> u64 {
    (v.checked_ilog10().unwrap() + 1) as u64
}

fn blink(cache: &mut FxHashMap<(u64, u64), u64>, v: u64, steps: u64) -> u64 {
    if let Some(computed_value) = cache.get(&(v, steps)) {
        return *computed_value;
    }

    let value = match (v, steps) {
        (_, 0) => 1,
        (0, steps) => blink(cache, 1, steps - 1),
        (v, steps) => {
            let num_digits = get_num_digits(v);
            if num_digits.is_multiple_of(2) {
                let first_half = v / 10_u64.pow(num_digits as u32 / 2);
                let second_half = v % 10_u64.pow(num_digits as u32 / 2);

                blink(cache, first_half, steps - 1) + blink(cache, second_half, steps - 1)
            } else {
                blink(cache, v * 2024, steps - 1)
            }
        }
    };

    cache.insert((v, steps), value);
    value
}

fn compute(input: &str, steps: u64) -> u64 {
    let mut cache: FxHashMap<(u64, u64), u64> = FxHashMap::default();

    input.split_ascii_whitespace().fold(0, |acc, v| {
        let v: u64 = v.parse().unwrap();
        acc + blink(&mut cache, v, steps)
    })
}

pub fn part1(input: &str) -> u64 {
    compute(input, 25)
}

pub fn part2(input: &str) -> u64 {
    compute(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        125 17
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 55312);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 198075);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 65601038650482);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 235571309320764);
    }
}
