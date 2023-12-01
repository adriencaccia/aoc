fn parse_input() -> Vec<u64> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().filter(|char| char.is_ascii_digit()))
        .map(|digits| {
            let vec: Vec<char> = digits.collect();
            let two_digits = format!(
                "{:?}{:?}",
                vec.first().unwrap().to_digit(10).unwrap(),
                vec.last().unwrap().to_digit(10).unwrap()
            );
            two_digits.parse().unwrap()
        })
        .collect()
}

pub fn main() -> u64 {
    let input = parse_input();

    let sum: u64 = input.iter().sum();

    println!("part1 {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(main(), 54644);
    }
}
