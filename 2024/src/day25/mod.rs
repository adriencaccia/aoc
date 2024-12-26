use arrayvec::ArrayVec;

const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const SIZE: usize = 2000;

pub fn part1(input: &str) -> u32 {
    let mut locks: ArrayVec<[u8; WIDTH], SIZE> = ArrayVec::new();
    let mut keys: ArrayVec<[u8; WIDTH], SIZE> = ArrayVec::new();

    input.split("\n\n").for_each(|pattern| {
        if &pattern.as_bytes()[..WIDTH] == b"#####" {
            let mut lock = [0; WIDTH];
            for line in pattern.lines().skip(1) {
                for (j, c) in line.bytes().enumerate() {
                    if c == b'#' {
                        lock[j] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = [0; WIDTH];
            for line in pattern.lines().take(HEIGHT - 1) {
                for (j, c) in line.bytes().enumerate() {
                    if c == b'#' {
                        key[j] += 1;
                    }
                }
            }
            keys.push(key);
        }
    });

    locks.into_iter().fold(0, |acc, lock| {
        acc + keys
            .iter()
            .filter(|&key| (0..WIDTH).all(|i| lock[i] + key[i] < (HEIGHT - 1) as u8))
            .count() as u32
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 3451);
    }
}
