use std::collections::HashMap;

/// in the inputs, the concatenated values have 3 digits max
#[inline(always)]
fn get_num_digits(v: u64) -> u64 {
    (v.checked_ilog10().unwrap() + 1) as u64
}

fn compute_blink(cache: &mut HashMap<(u64, u64), u64>, v: u64, steps: u64) -> u64 {
    if let Some(computed_value) = cache.get(&(v, steps)) {
        return *computed_value;
    }

    let value = match (v, steps) {
        (_, 0) => 1,
        (0, steps) => compute_blink(cache, 1, steps - 1),
        (v, steps) => {
            let num_digits = get_num_digits(v);
            if num_digits % 2 == 0 {
                let first_half = v / 10_u64.pow(num_digits as u32 / 2);
                let second_half = v % 10_u64.pow(num_digits as u32 / 2);

                compute_blink(cache, first_half, steps - 1)
                    + compute_blink(cache, second_half, steps - 1)
            } else {
                compute_blink(cache, v * 2024, steps - 1)
            }
        }
    };

    cache.insert((v, steps), value);
    value
}

pub fn part1(input: &str) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    input.split_whitespace().fold(0, |acc, v| {
        let v: u64 = v.parse().unwrap();

        acc + compute_blink(&mut cache, v, 25)
    })
}

pub fn part2(input: &str) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    input.split_whitespace().fold(0, |acc, v| {
        let v: u64 = v.parse().unwrap();

        acc + compute_blink(&mut cache, v, 75)
    })
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
