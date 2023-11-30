fn parse_input() -> Vec<u64> {
    include_str!("input.txt")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

pub fn main() {
    let input = parse_input();

    let sum = input.iter().sum::<u64>();

    println!("sum {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
