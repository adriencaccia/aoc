use itertools::Itertools;
use rustc_hash::FxHashMap;

enum Operation {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

struct Wire {
    value: bool,
    operation: Option<Operation>,
}

fn solve_wire(wires: &FxHashMap<String, Wire>, wire: &str) -> bool {
    if let Some(wire) = wires.get(wire) {
        if let Some(operation) = &wire.operation {
            match operation {
                Operation::And(a, b) => solve_wire(wires, a) & solve_wire(wires, b),
                Operation::Or(a, b) => solve_wire(wires, a) | solve_wire(wires, b),
                Operation::Xor(a, b) => solve_wire(wires, a) ^ solve_wire(wires, b),
            }
        } else {
            wire.value
        }
    } else {
        false
    }
}

pub fn part1(input: &str) -> u64 {
    let mut wires: FxHashMap<String, Wire> = FxHashMap::default();

    let mut it = input.split("\n\n");

    for line in it.next().unwrap().lines() {
        let mut parts = line.split(": ");
        let wire = parts.next().unwrap();
        let value = parts.next().unwrap();

        wires.insert(
            wire.to_string(),
            Wire {
                value: value == "1",
                operation: None,
            },
        );
    }

    let mut zs = vec![];

    for line in it.next().unwrap().lines() {
        let mut parts = line.split(" -> ");
        let operation = parts.next().unwrap();
        let wire = parts.next().unwrap();

        let operation = match operation.split(" ").collect_tuple().unwrap() {
            (a, "AND", b) => Operation::And(a.to_string(), b.to_string()),
            (a, "OR", b) => Operation::Or(a.to_string(), b.to_string()),
            (a, "XOR", b) => Operation::Xor(a.to_string(), b.to_string()),
            _ => unreachable!(),
        };

        wires.insert(
            wire.to_string(),
            Wire {
                value: false,
                operation: Some(operation),
            },
        );
        if wire.starts_with("z") {
            zs.push(wire.to_string());
        }
    }

    zs.sort();
    zs.into_iter().fold(0, |acc, z| {
        let value = solve_wire(&wires, &z);

        if value {
            acc | 1 << z[1..].parse::<u64>().unwrap()
        } else {
            acc
        }
    })
}

pub fn part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2024);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), 48806532300520);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), "");
    }
}
