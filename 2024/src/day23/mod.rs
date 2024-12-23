use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::FxHashSet;

const N_CHARS: usize = 26;
const SIZE: usize = N_CHARS * N_CHARS;
// const N_SETS: usize = SIZE * SIZE + SIZE * N_CHARS + SIZE;

fn starts_with_t(index: usize) -> bool {
    index / N_CHARS == (b't' - b'a') as usize
}

pub fn part1(input: &str) -> u32 {
    let mut adj = [[0u8; SIZE]; SIZE]; // might be able to reduce this size
    let mut vertices = [0u8; SIZE];

    input.trim_ascii().lines().for_each(|line| {
        let (a, b) = line
            .split("-")
            .map(|s| {
                let (a, b) = s.as_bytes().iter().collect_tuple().unwrap();
                (a - b'a') as usize * N_CHARS + (b - b'a') as usize
            })
            .collect_tuple()
            .unwrap();

        vertices[a] = 1;
        vertices[b] = 1;

        adj[a][b] = 1;
        adj[b][a] = 1;
    });

    // let mut sets = [0; N_SETS]; // stores the set of 3 vertices that are connected to each other, with vertices sorted in increasing order
    let mut sets = FxHashSet::default();

    for (vertex_index, present) in vertices.iter().enumerate() {
        if *present == 0 {
            continue;
        }

        /// Because we are DFSing and each vertex has at most 13 neighbors, we can use a stack
        /// with a size of 24 to avoid heap allocation
        const MAX_STACK_SIZE: usize = 24;
        let mut stack = ArrayVec::<(usize, usize, [usize; 3]), MAX_STACK_SIZE>::new();
        stack.push((vertex_index, 1, [vertex_index, usize::MAX, usize::MAX]));

        while let Some((vertex, depth, path)) = stack.pop() {
            for (neighbor, &connected) in adj[vertex].iter().enumerate() {
                if connected == 0 {
                    continue;
                }

                if depth == 3 {
                    if neighbor != vertex_index {
                        continue;
                    }

                    let mut set = path;
                    if !set.iter().any(|&v| starts_with_t(v)) {
                        break;
                    }
                    set.sort_unstable();
                    // TODO: move this in a Computer struct
                    // dbg!(set.map(|v| format!(
                    //     "{}{}",
                    //     (((v / N_CHARS) as u8 + b'a') as char),
                    //     (((v % N_CHARS) as u8 + b'a') as char)
                    // )));
                    let set_index = set[0] * SIZE * SIZE + set[1] * SIZE + set[2];
                    sets.insert(set_index);
                    break;
                }

                if path[0] == neighbor || path[1] == neighbor || neighbor == vertex_index {
                    continue;
                }

                let mut new_path = path;
                new_path[depth] = neighbor;
                stack.push((neighbor, depth + 1, new_path));
            }
        }
    }
    // TODO: find out why there is one cycle missing in the example input when not filtering out sets that don't contain a vertex starting with 't'
    // sets.iter().for_each(|v| {
    //     let a = v / (SIZE * SIZE);
    //     let a1 = format!(
    //         "{}{}",
    //         (((a / N_CHARS) as u8 + b'a') as char),
    //         (((a % N_CHARS) as u8 + b'a') as char)
    //     );
    //     let b = (v - a * (SIZE * SIZE)) / SIZE;
    //     let b1 = format!(
    //         "{}{}",
    //         (((b / N_CHARS) as u8 + b'a') as char),
    //         (((b % N_CHARS) as u8 + b'a') as char)
    //     );
    //     let c = v - (a * (SIZE * SIZE)) - b * SIZE;
    //     let c1 = format!(
    //         "{}{}",
    //         (((c / N_CHARS) as u8 + b'a') as char),
    //         (((c % N_CHARS) as u8 + b'a') as char)
    //     );
    //     println!("{},{},{} - {}", a1, b1, c1, v);
    // });

    sets.len() as u32
}

pub fn part2(input: &str) -> String {
    let mut adj = [[0u8; SIZE]; SIZE]; // might be able to reduce this size
    let mut vertices = [0u8; SIZE];

    input.trim_ascii().lines().for_each(|line| {
        let (a, b) = line
            .split("-")
            .map(|s| {
                let (a, b) = s.as_bytes().iter().collect_tuple().unwrap();
                (a - b'a') as usize * N_CHARS + (b - b'a') as usize
            })
            .collect_tuple()
            .unwrap();

        vertices[a] = 1;
        vertices[b] = 1;

        adj[a][b] = 1;
        adj[b][a] = 1;
    });

    let mut cliques: FxHashSet<ArrayVec<usize, SIZE>> = FxHashSet::default();

    for (vertex_index, present) in vertices.iter().enumerate() {
        if *present == 0 {
            continue;
        }
        let mut clique = ArrayVec::<usize, SIZE>::new();
        clique.push(vertex_index);
        for (vertex, present) in vertices.iter().enumerate() {
            if *present == 0 || vertex == vertex_index {
                continue;
            }
            let is_in_clique = clique.iter().all(|&v| adj[vertex][v] == 1);
            if is_in_clique {
                clique.push(vertex);
            }
        }
        clique.sort();
        cliques.insert(clique);
    }

    let max_clique = cliques.iter().max_by_key(|c| c.len()).unwrap();

    max_clique
        .iter()
        .map(|&v| {
            format!(
                "{}{}",
                (((v / N_CHARS) as u8 + b'a') as char),
                (((v % N_CHARS) as u8 + b'a') as char)
            )
        })
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 1062);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), "co,de,ka,ta");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(include_str!("input.txt")),
            "bz,cs,fx,ms,oz,po,sy,uh,uv,vw,xu,zj,zm"
        );
    }
}
