#[derive(Debug)]
struct File {
    position: u64,
    id: u64,
    blocks: u64,
}

#[derive(Debug)]
struct EmptySpace {
    position: u64,
    blocks: u64,
}

const CAPACITY: usize = 10_000;

fn parse_input(input: &str) -> (Vec<File>, Vec<EmptySpace>) {
    let mut files: Vec<File> = Vec::with_capacity(CAPACITY);
    let mut empty_spaces: Vec<EmptySpace> = Vec::with_capacity(CAPACITY);

    let mut id = 0;
    let mut position = 0;
    input
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .for_each(|(i, c)| {
            let is_file = i % 2 == 0;
            let blocks = (c - b'0') as u64;
            if is_file {
                files.push(File {
                    position,
                    id,
                    blocks,
                });
                id += 1;
            } else {
                empty_spaces.push(EmptySpace { position, blocks });
            }
            position += blocks;
        });
    empty_spaces.pop(); // TODO: find a way to not have to do this

    (files, empty_spaces)
}

fn checksum(files: &[File]) -> u64 {
    files.iter().fold(
        0,
        |sum,
         File {
             blocks,
             id,
             position,
             // simplified version of the following sum, using MATHS 🏆
             // sum + (0..blocks).fold(0, |acc, k| acc + id * (position + k)),
         }| sum + id * blocks * (2 * position + blocks - 1) / 2,
    )
}

pub fn part1(input: &str) -> u64 {
    let (mut files, empty_spaces) = parse_input(input);

    'outer: for empty_space in empty_spaces {
        let EmptySpace {
            mut position,
            mut blocks,
        } = empty_space;
        while blocks > 0 {
            let last_file = files.pop().unwrap();

            if last_file.position < position {
                files.push(last_file);
                break 'outer;
            }

            // last file has more blocks
            if last_file.blocks > blocks {
                // push same last file with less blocks
                files.push(File {
                    blocks: last_file.blocks - blocks,
                    ..last_file
                });
                // prepend new last file with blocks
                files.insert(
                    0,
                    File {
                        blocks,
                        id: last_file.id,
                        position,
                    },
                );
                blocks = 0;
            } else {
                // last file has less blocks
                // prepend new last file with blocks
                files.insert(
                    0,
                    File {
                        position,
                        ..last_file
                    },
                );
                blocks -= last_file.blocks;
                position += last_file.blocks;
            }
        }
    }

    checksum(&files)
}

pub fn part2(input: &str) -> u64 {
    let (mut files, mut empty_spaces) = parse_input(input);

    let files_len = files.len();
    let mut new_files: Vec<File> = Vec::with_capacity(files_len);
    for _ in 0..files_len {
        let file_to_move = files.pop().unwrap();
        // find the first empty space that can fit the file
        let empty_space = empty_spaces
            .iter_mut()
            .find(|es| es.blocks >= file_to_move.blocks && es.position <= file_to_move.position);
        if let Some(empty_space) = empty_space {
            empty_space.blocks -= file_to_move.blocks;
            new_files.push(File {
                position: empty_space.position,
                ..file_to_move
            });
            empty_space.position += file_to_move.blocks;
        } else {
            new_files.push(file_to_move);
        }
    }

    checksum(&new_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        2333133121414131402
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1928);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 6385338159127);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 2858);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 6415163624282);
    }
}
