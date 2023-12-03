fn parse_input() -> impl Iterator<Item = u32> {
    include_str!("input.txt").lines().map(|line| {
        let digits = line.chars().filter_map(|char| char.to_digit(10));
        digits.clone().next().unwrap() * 10 + digits.last().unwrap()
    })
}

fn parse_input_part_2() -> Vec<u64> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();

            let mut digits: Vec<u64> = vec![];
            let mut i = 0;
            while i < chars.len() {
                if let Some(digit) = chars[i].to_digit(10) {
                    digits.push(digit.into());
                    i += 1;
                    continue;
                }
                if (i + 3) <= chars.len() && chars[i..i + 3].iter().collect::<String>() == "one" {
                    digits.push(1);
                    i += 1;
                    continue;
                }
                if (i + 3) <= chars.len() && chars[i..i + 3].iter().collect::<String>() == "two" {
                    digits.push(2);
                    i += 1;
                    continue;
                }
                if (i + 5) <= chars.len() && chars[i..i + 5].iter().collect::<String>() == "three" {
                    digits.push(3);
                    i += 1;
                    continue;
                }
                if (i + 4) <= chars.len() && chars[i..i + 4].iter().collect::<String>() == "four" {
                    digits.push(4);
                    i += 1;
                    continue;
                }
                if (i + 4) <= chars.len() && chars[i..i + 4].iter().collect::<String>() == "five" {
                    digits.push(5);
                    i += 1;
                    continue;
                }
                if (i + 3) <= chars.len() && chars[i..i + 3].iter().collect::<String>() == "six" {
                    digits.push(6);
                    i += 1;
                    continue;
                }
                if (i + 5) <= chars.len() && chars[i..i + 5].iter().collect::<String>() == "seven" {
                    digits.push(7);
                    i += 1;
                    continue;
                }
                if (i + 5) <= chars.len() && chars[i..i + 5].iter().collect::<String>() == "eight" {
                    digits.push(8);
                    i += 1;
                    continue;
                }
                if (i + 4) <= chars.len() && chars[i..i + 4].iter().collect::<String>() == "nine" {
                    digits.push(9);
                    i += 1;
                    continue;
                }
                i += 1;
            }

            let two_digits = format!("{:?}{:?}", digits.first().unwrap(), digits.last().unwrap());
            two_digits.parse().unwrap()
        })
        .collect()
}

pub fn main() -> (u32, u64) {
    let input = parse_input();
    let part1 = input.sum();
    println!("part1 {}", part1);

    let input2 = parse_input_part_2();
    let part2: u64 = input2.iter().sum();
    println!("part2 {}", part2);

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(main().0, 54644);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(main().1, 53348);
    }
}
