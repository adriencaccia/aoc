use arrayvec::ArrayVec;
use itertools::Itertools;
use rustc_hash::FxHashSet;

const N_CHARS: usize = 26;
const SIZE: usize = N_CHARS * N_CHARS;

fn starts_with_t(index: usize) -> bool {
    index / N_CHARS == (b't' - b'a') as usize
}

fn parse(input: &str) -> ([[u8; SIZE]; SIZE], [u8; SIZE]) {
    let mut adj = [[0u8; SIZE]; SIZE];
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

    (adj, vertices)
}

pub fn part1(input: &str) -> u32 {
    let (adj, vertices) = parse(input);
    let mut total = 0;

    for (x, present) in vertices.iter().enumerate() {
        if *present == 0 {
            continue;
        }
        for (y, &connected) in adj[x].iter().enumerate() {
            if connected == 0 {
                continue;
            }
            for (z, &connected) in adj[y].iter().enumerate() {
                if connected == 0 {
                    continue;
                }
                if adj[z][x] == 1 && [x, y, z].iter().any(|&v| starts_with_t(v)) {
                    total += 1;
                }
            }
        }
    }

    // all triangles are counted 6 times each
    total / 6
}

pub fn part2(input: &str) -> String {
    let (adj, vertices) = parse(input);
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
