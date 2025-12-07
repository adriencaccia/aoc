pub fn part1(input: &str) -> usize {
    let mut lines: Vec<_> = input.trim_ascii().lines().collect();

    let sign_line = lines.pop().unwrap();
    let signs: Vec<char> = sign_line
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let numbers: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    // transpose numbers
    let mut transposed: Vec<Vec<usize>> = vec![vec![]; numbers[0].len()];
    for row in &numbers {
        for (j, &num) in row.iter().enumerate() {
            transposed[j].push(num);
        }
    }

    let mut sum_results = 0;
    for (col_idx, col) in transposed.iter().enumerate() {
        let sign = signs[col_idx];
        let col_sum: usize = match sign {
            '+' => col.iter().sum(),
            '*' => col.iter().product(),
            _ => panic!("unknown operator"),
        };
        sum_results += col_sum;
    }

    sum_results
}

pub fn part2(input: &str) -> usize {
    let mut lines: Vec<_> = input.lines().collect();
    let sign_line_str = lines.pop().unwrap();
    let sign_chars: Vec<char> = sign_line_str.chars().collect();

    // (operator, start_index, end_index)
    let operations: Vec<(char, usize, usize)> = sign_chars
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c == '+' || c == '*' {
                // end index is the next operator or end of line
                let end_index = sign_chars[i + 1..]
                    .iter()
                    .position(|&c| c == '+' || c == '*')
                    .map_or(sign_chars.len() - 1, |pos| pos + i - 1);
                Some((c, i, end_index))
            } else {
                None
            }
        })
        .collect();

    operations.iter().fold(0, |sum, &(op, start, end)| {
        let nums: Vec<_> = (start..=end)
            .map(|i| {
                lines.iter().fold(0, |line_acc, line| {
                    if let Some(c) = line.chars().nth(i)
                        && c.is_ascii_digit()
                    {
                        line_acc * 10 + c.to_digit(10).unwrap() as usize
                    } else {
                        line_acc
                    }
                })
            })
            .collect();

        sum + match op {
            '+' => nums.iter().sum::<usize>(),
            '*' => nums.iter().product::<usize>(),
            _ => panic!("unknown operator"),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 4277556);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 5322004718681);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 3263827);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 9876636978528);
    }
}
