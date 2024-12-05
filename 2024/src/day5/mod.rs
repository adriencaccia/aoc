const UPDATE_MAX_SIZE: usize = 23; // real size is 23, we store 2 extra values. 1 for the middle value and 1 for the size of the update
const UPDATES_LEN: usize = 187;
const MAX_SIZE: usize = 100;
type AdjMatrix = [[bool; MAX_SIZE]; MAX_SIZE];

fn parse(input: &str) -> (AdjMatrix, [[u8; UPDATE_MAX_SIZE + 2]; UPDATES_LEN]) {
    let mut it = input.split("\n\n");
    let mut matrix = [[false; MAX_SIZE]; MAX_SIZE];

    it.next().unwrap().lines().for_each(|l| {
        let mut parts = l.split('|');
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        matrix[a][b] = true;
    });

    let mut updates = [[0; UPDATE_MAX_SIZE + 2]; UPDATES_LEN];
    it.next().unwrap().lines().enumerate().for_each(|(i, l)| {
        let mut peekable = l.split(',').peekable();
        let mut j = 0;
        while let Some(v) = peekable.peek() {
            updates[i][j] = v.parse().unwrap();
            j += 1;
            peekable.next();
        }
        updates[i][UPDATE_MAX_SIZE] = updates[i][j / 2];
        updates[i][UPDATE_MAX_SIZE + 1] = j as u8;
    });

    (matrix, updates)
}

fn is_update_correct_order(update: &[u8], matrix: &AdjMatrix) -> bool {
    for i in 0..UPDATE_MAX_SIZE - 1 {
        let a = update[i];
        for &b in &update[i + 1..UPDATE_MAX_SIZE] {
            if b == 0 {
                break;
            }
            if matrix[b as usize][a as usize] {
                return false;
            }
        }
    }

    true
}

pub fn part1(input: &str) -> u16 {
    let (matrix, updates) = parse(input);

    updates.into_iter().fold(0, |acc, update| {
        acc + if is_update_correct_order(&update, &matrix) {
            update[UPDATE_MAX_SIZE] as u16
        } else {
            0
        }
    })
}

fn sort_update(update: &mut [u8], matrix: &AdjMatrix) {
    let len = update[UPDATE_MAX_SIZE + 1] as usize;
    for _i in 0..len {
        for j in 0..len - 1 {
            let a = update[j] as usize;
            let b = update[j + 1] as usize;
            if matrix[b][a] {
                update.swap(j, j + 1);
            }
        }
    }
    update[UPDATE_MAX_SIZE] = update[len / 2];
}

pub fn part2(input: &str) -> u16 {
    let (matrix, updates) = parse(input);

    updates.into_iter().fold(0, |acc, mut update| {
        if is_update_correct_order(&update, &matrix) {
            return acc;
        }
        sort_update(&mut update, &matrix);
        acc + update[UPDATE_MAX_SIZE] as u16
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 143);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 4578);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 123);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 6179);
    }
}
