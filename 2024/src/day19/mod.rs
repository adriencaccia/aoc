use rustc_hash::FxHashMap;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut it = input.split("\n\n");

    let mut towels: Vec<&str> = Vec::with_capacity(500);
    towels.extend(it.next().unwrap().split(", "));
    let mut designs: Vec<&str> = Vec::with_capacity(400);
    designs.extend(it.next().unwrap().trim_ascii().split("\n"));

    (towels, designs)
}

fn possible(towels: &[&str], cache: &mut FxHashMap<String, bool>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    if cache.contains_key(design) {
        return *cache.get(design).unwrap();
    }

    let is_possible = towels.iter().any(|t| {
        if design.starts_with(t) {
            possible(towels, cache, design.strip_prefix(t).unwrap())
        } else {
            false
        }
    });
    cache.insert(design.to_string(), is_possible);

    is_possible
}

pub fn part1(input: &str) -> u32 {
    let (towels, designs) = parse(input);
    let mut cache = FxHashMap::default();

    designs.iter().fold(0, |acc, design| {
        if possible(&towels, &mut cache, design) {
            acc + 1
        } else {
            acc
        }
    })
}

fn combinations(towels: &[&str], cache: &mut FxHashMap<String, u64>, design: &str) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if cache.contains_key(design) {
        return *cache.get(design).unwrap();
    }

    let count = towels.iter().fold(0, |acc, t| {
        if design.starts_with(t) {
            acc + combinations(towels, cache, design.strip_prefix(t).unwrap())
        } else {
            acc
        }
    });
    cache.insert(design.to_string(), count);

    count
}

pub fn part2(input: &str) -> u64 {
    let (towels, designs) = parse(input);
    let mut cache = FxHashMap::default();

    designs.iter().fold(0, |acc, design| {
        acc + combinations(&towels, &mut cache, design)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 293);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 16);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 623924810770264);
    }
}
