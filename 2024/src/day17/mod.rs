use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Hash, Debug, Clone)]
#[display(
    "Register A: {a}
Register B: {b}
Register C: {c}"
)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

const SIZE: usize = 16;
const OUTPUT_SIZE: usize = 100;

fn combo(o: u8, registers: &Registers) -> u64 {
    match o {
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        7 => unreachable!(),
        o => o as u64,
    }
}

pub fn part1(input: &str) -> String {
    let mut it = input.split("\n\n");
    let mut registers: Registers = it.next().unwrap().parse().unwrap();
    let mut program = Vec::with_capacity(SIZE);
    program.extend(
        it.next()
            .unwrap()
            .bytes()
            .skip(9)
            .filter(|&b| b != b',' && b != b'\n')
            .map(|b| b - b'0'),
    );

    let mut pointer = 0;
    let mut output = Vec::with_capacity(OUTPUT_SIZE);

    while pointer < program.len() {
        if let Some(o) = execute_step(&mut registers, &program, &mut pointer) {
            output.push(o);
        }
    }

    output
        .iter()
        .map(|&o| o.to_string())
        .collect_vec()
        .join(",")
}

fn execute_step(registers: &mut Registers, program: &[u8], pointer: &mut usize) -> Option<u64> {
    let o = program[*pointer + 1];
    match program[*pointer] {
        0 => {
            registers.a >>= combo(o, &*registers);
        }
        1 => {
            registers.b ^= o as u64;
        }
        2 => {
            registers.b = combo(o, &*registers) & 7; // same as % 8
        }
        3 => {
            if registers.a != 0 {
                *pointer = o as usize;
                return None;
            }
        }
        4 => {
            registers.b ^= registers.c;
        }
        5 => {
            *pointer += 2;
            return Some(combo(o, &*registers) & 7); // same as % 8
        }
        6 => {
            registers.b = registers.a >> combo(o, &*registers);
        }
        7 => {
            registers.c = registers.a >> combo(o, &*registers);
        }
        _ => unreachable!(),
    };
    *pointer += 2;
    None
}

// only works for my input
// fn find(program: &[u8], ans: u64) -> Option<u64> {
//     if program.is_empty() {
//         return Some(ans);
//     }
//     for by in 0..8 {
//         let a = ans << 3 | by;
//         let mut b = a & 7;
//         b ^= 1;
//         let c = a >> b;
//         b ^= c;
//         b ^= 6;
//         if (b & 7) as u8 == *program.last().unwrap() {
//             if let Some(sub) = find(&program[..program.len() - 1], a) {
//                 return Some(sub);
//             }
//             continue;
//         }
//     }
//     None
// }

fn find_for_all(program: &[u8], target: &[u8], ans: u64) -> Option<u64> {
    if target.is_empty() {
        return Some(ans);
    }
    for by in 0..8 {
        let mut registers = Registers {
            a: (ans << 3) | by,
            b: 0,
            c: 0,
        };

        let mut out = None;

        for pointer in (0..(program.len() - 2)).step_by(2) {
            let mut dum_pointer = pointer;
            // skip 0 opcode as it handled by the recursion
            if program[pointer] != 0 {
                if let Some(new_out) = execute_step(&mut registers, program, &mut dum_pointer) {
                    out = Some(new_out);
                }
            }

            if let Some(out) = out {
                if out == (target[target.len() - 1] as u64) {
                    if let Some(sub) =
                        find_for_all(program, &target[..(target.len() - 1)], registers.a)
                    {
                        return Some(sub);
                    }
                }
            }
        }
    }
    None
}

pub fn part2(input: &str) -> u64 {
    let mut it = input.split("\n\n");
    let mut program = Vec::with_capacity(SIZE);
    program.extend(
        it.nth(1)
            .unwrap()
            .bytes()
            .skip(9)
            .filter(|&b| b != b',' && b != b'\n')
            .map(|b| b - b'0'),
    );

    // find(&program, 0).unwrap()
    find_for_all(&program, &program, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_INPUT_1: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
"};

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("input.txt")), "2,7,2,5,1,2,7,3,7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("input.txt")), 247839002892474);
    }
}
