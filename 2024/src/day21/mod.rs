use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::{collections::VecDeque, iter::zip};

fn compute_sequences(keypad: &[[u8; 3]]) -> FxHashMap<(u8, u8), Vec<String>> {
    let mut pos = FxHashMap::default();
    keypad.iter().enumerate().for_each(|(x, row)| {
        for (y, &b) in row.iter().enumerate() {
            if b == b' ' {
                continue;
            }
            pos.insert(b, (x, y));
        }
    });

    // required moves to go from one key to another
    let mut sequences = FxHashMap::default();
    for x in pos.keys() {
        for y in pos.keys() {
            if *x == *y {
                sequences.insert((*x, *y), vec!["A".to_string()]);
                continue;
            }
            let mut possibilities = vec![];
            let mut min_length = usize::MAX;
            let mut stack = VecDeque::from([(pos[x], "".to_string())]);

            'outer: while let Some(((r, c), moves)) = stack.pop_front() {
                let ri = r as isize;
                let ci = c as isize;

                for (nr, nc, nm) in [
                    (ri - 1, ci, "^"),
                    (ri + 1, ci, "v"),
                    (ri, ci - 1, "<"),
                    (ri, ci + 1, ">"),
                ] {
                    if nr < 0
                        || nc < 0
                        || nr as usize >= keypad.len()
                        || nc as usize >= keypad[nr as usize].len()
                    {
                        continue;
                    }
                    let nx = nr as usize;
                    let ny = nc as usize;
                    if keypad[nx][ny] == b' ' {
                        continue;
                    }
                    if keypad[nx][ny] == *y {
                        if min_length < moves.len() + 1 {
                            break 'outer;
                        }
                        min_length = moves.len() + 1;
                        possibilities.push(moves.clone() + nm + "A");
                    } else {
                        stack.push_back(((nx, ny), moves.clone() + nm));
                    }
                }
            }

            sequences.insert((*x, *y), possibilities);
        }
    }

    sequences
}

fn generate_sequences(string: &str, sequences: &FxHashMap<(u8, u8), Vec<String>>) -> Vec<String> {
    let options: Vec<Vec<String>> = zip(format!("A{}", string).bytes(), string.bytes())
        .map(|(x, y)| sequences[&(x, y)].clone())
        .collect();

    options
        .iter()
        .multi_cartesian_product()
        .map(|x| x.into_iter().join(""))
        .collect()
}

fn compute_length(
    seq: String,
    depth: u8,
    dir_sequences: &FxHashMap<(u8, u8), Vec<String>>,
    dir_lengths: &FxHashMap<(u8, u8), u8>,
    cache: &mut FxHashMap<(String, u8), u64>,
) -> u64 {
    if depth == 1 {
        return zip(format!("A{}", seq).bytes(), seq.bytes())
            .map(|(x, y)| dir_lengths[&(x, y)] as u64)
            .sum();
    }
    if let Some(&length) = cache.get(&(seq.clone(), depth)) {
        return length;
    }

    let length = zip(format!("A{}", seq).bytes(), seq.bytes()).fold(0, |acc, (x, y)| {
        acc + dir_sequences[&(x, y)]
            .iter()
            .map(|s| compute_length(s.clone(), depth - 1, dir_sequences, dir_lengths, cache))
            .min()
            .unwrap()
    });
    cache.insert((seq, depth), length);

    length
}

fn solve(input: &str, depth: u8) -> u64 {
    let num_keypad: Vec<[u8; 3]> = vec![
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b' ', b'0', b'A'],
    ];
    let dir_keypad: Vec<[u8; 3]> = vec![[b' ', b'^', b'A'], [b'<', b'v', b'>']];
    let num_sequences = compute_sequences(&num_keypad);
    let dir_sequences = compute_sequences(&dir_keypad);
    let dir_lengths = FxHashMap::from(
        dir_sequences
            .iter()
            .map(|(k, v)| (*k, v[0].len() as u8))
            .collect(),
    );

    let mut cache = FxHashMap::default();

    input.trim().lines().fold(0, |acc, line| {
        let inputs = generate_sequences(line, &num_sequences);
        let length = inputs
            .into_iter()
            .map(|s| compute_length(s, depth, &dir_sequences, &dir_lengths, &mut cache))
            .min()
            .unwrap();

        acc + length * line[..line.len() - 1].parse::<u64>().unwrap()
    })
}

pub fn part1(input: &str) -> u64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 126384);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 164960);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 154115708116294);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 205620604017764);
    }
}
