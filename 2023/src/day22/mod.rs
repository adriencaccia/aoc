fn parse_input(input: &str) -> (u32, u32) {
    let _lines = input.trim().lines();

    // parse bricks
    // sort them by their first z coordinate
    // iterate over them and place them at the lowest possible z coordinate
    //   record which bricks stopped the current brick from moving

    // for each brick, check if it is not the only one in all of the others bricks support
    // return the number of bricks from ⬆️

    let part1 = 0;
    let part2 = 0;
    (part1, part2)
}

pub fn main() -> (u32, u32) {
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
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 5);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 0);
        assert_eq!(part2, 0);
    }
}
