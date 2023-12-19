use std::{collections::HashMap, ops::RangeInclusive};

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Debug)]
enum Rule {
    #[display("{part_to_cmp}<{target}:{destination}")]
    LessThan {
        part_to_cmp: String,
        target: usize,
        destination: String,
    },
    #[display("{part_to_cmp}>{target}:{destination}")]
    GreaterThan {
        part_to_cmp: String,
        target: usize,
        destination: String,
    },
    #[display("{destination}")]
    GoTo { destination: String },
}

#[derive(Display, FromStr, Eq, PartialEq, Debug)]
#[display("{name}{{{rules}}}")]
struct Workflow {
    name: String,
    rules: String,
}

#[derive(Eq, PartialEq, Debug)]
struct ParsedWorkflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<Workflow> for ParsedWorkflow {
    fn from(workflow: Workflow) -> Self {
        let rules = workflow
            .rules
            .split(',')
            .map(|rule| rule.parse().unwrap())
            .collect();

        Self {
            name: workflow.name,
            rules,
        }
    }
}

#[derive(Display, FromStr, Eq, PartialEq, Debug)]
#[display("{{x={x},m={m},a={a},s={s}}}")]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

// find all paths between 'in' and 'A', recording the rule index for each step
// TODO: change to a recursive function that starts with the whole range and split it for every possibility, return sum the combinaisons of the if reaching the 'A' workflow, otherwise return 0
fn find_all_paths(workflows: &HashMap<String, &ParsedWorkflow>) -> Vec<Vec<(String, usize)>> {
    let mut paths = vec![];
    let mut queue = vec![vec![("in".to_string(), 0)]];

    while let Some(path) = queue.pop() {
        let (last_workflow, _) = path.last().unwrap();
        let last_workflow_rules = &workflows.get(last_workflow).unwrap().rules;

        for (idx, rule) in last_workflow_rules.iter().enumerate() {
            match rule {
                Rule::LessThan {
                    part_to_cmp: _,
                    target: _,
                    destination,
                } => {
                    let mut new_path = path.clone();
                    new_path.push((destination.clone(), idx));
                    queue.push(new_path);
                }
                Rule::GreaterThan {
                    part_to_cmp: _,
                    target: _,
                    destination,
                } => {
                    let mut new_path = path.clone();
                    new_path.push((destination.clone(), idx));
                    queue.push(new_path);
                }
                Rule::GoTo { destination } => {
                    let mut new_path = path.clone();
                    new_path.push((destination.clone(), idx));
                    queue.push(new_path);
                }
            }
        }

        if last_workflow == "A" {
            paths.push(path);
        }
    }

    paths
}

fn get_accepted_part_ranges(
    workflows: &HashMap<String, &ParsedWorkflow>,
    path: Vec<(String, usize)>,
) -> (
    RangeInclusive<usize>,
    RangeInclusive<usize>,
    RangeInclusive<usize>,
    RangeInclusive<usize>,
) {
    let mut x_range = 1..=4000;
    let mut m_range = 1..=4000;
    let mut a_range = 1..=4000;
    let mut s_range = 1..=4000;

    for ((current, _), (next, next_idx)) in path.iter().tuple_windows() {
        let workflow = workflows.get(current).unwrap();

        for (idx, rule) in workflow.rules.iter().enumerate() {
            match rule {
                Rule::LessThan {
                    part_to_cmp,
                    target,
                    destination,
                } => {
                    if destination == next && idx == *next_idx {
                        match part_to_cmp.as_str() {
                            "x" => x_range = *x_range.start()..=(target - 1).min(*x_range.end()),
                            "m" => m_range = *m_range.start()..=(target - 1).min(*m_range.end()),
                            "a" => a_range = *a_range.start()..=(target - 1).min(*a_range.end()),
                            "s" => s_range = *s_range.start()..=(target - 1).min(*s_range.end()),
                            _ => unreachable!(),
                        };
                        break;
                    }
                    match part_to_cmp.as_str() {
                        "x" => x_range = *target.max(x_range.start())..=*x_range.end(),
                        "m" => m_range = *target.max(m_range.start())..=*m_range.end(),
                        "a" => a_range = *target.max(a_range.start())..=*a_range.end(),
                        "s" => s_range = *target.max(s_range.start())..=*s_range.end(),
                        _ => unreachable!(),
                    };
                }
                Rule::GreaterThan {
                    part_to_cmp,
                    target,
                    destination,
                } => {
                    if destination == next && idx == *next_idx {
                        match part_to_cmp.as_str() {
                            "x" => x_range = *target.max(x_range.start()) + 1..=*x_range.end(),
                            "m" => m_range = *target.max(m_range.start()) + 1..=*m_range.end(),
                            "a" => a_range = *target.max(a_range.start()) + 1..=*a_range.end(),
                            "s" => s_range = *target.max(s_range.start()) + 1..=*s_range.end(),
                            _ => unreachable!(),
                        };
                        break;
                    }
                    match part_to_cmp.as_str() {
                        "x" => x_range = *x_range.start()..=*target.min(x_range.end()),
                        "m" => m_range = *m_range.start()..=*target.min(m_range.end()),
                        "a" => a_range = *a_range.start()..=*target.min(a_range.end()),
                        "s" => s_range = *s_range.start()..=*target.min(s_range.end()),
                        _ => unreachable!(),
                    };
                }
                _ => {}
            };
        }
    }

    (x_range, m_range, a_range, s_range)
}

fn parse_input(input: &str) -> (usize, usize) {
    let (workflows_str, parts_str) = input.split("\n\n").collect_tuple().unwrap();

    let workflows: Vec<ParsedWorkflow> = workflows_str
        .lines()
        .map(|line| line.parse::<Workflow>().unwrap().into())
        .collect();
    let mut workflows_map: HashMap<String, &ParsedWorkflow> = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();
    let accepted_workflow = ParsedWorkflow {
        name: "A".to_string(),
        rules: vec![],
    };
    workflows_map.insert("A".to_string(), &accepted_workflow);
    let rejected_workflow = ParsedWorkflow {
        name: "R".to_string(),
        rules: vec![],
    };
    workflows_map.insert("R".to_string(), &rejected_workflow);
    let parts: Vec<Part> = parts_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let accepted_parts = parts.iter().filter(|part| {
        let mut queue = vec![*workflows_map.get("in").unwrap()];

        while let Some(workflow) = queue.pop() {
            if workflow.name == 'A'.to_string() {
                return true;
            }
            if workflow.name == 'R'.to_string() {
                return false;
            }

            for rule in workflow.rules.iter() {
                match rule {
                    Rule::LessThan {
                        part_to_cmp,
                        target,
                        destination,
                    } => {
                        let is_part_valid = match part_to_cmp.as_str() {
                            "x" => part.x < *target,
                            "m" => part.m < *target,
                            "a" => part.a < *target,
                            "s" => part.s < *target,
                            _ => unreachable!(),
                        };
                        if is_part_valid {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                    }
                    Rule::GreaterThan {
                        part_to_cmp,
                        target,
                        destination,
                    } => {
                        let is_part_valid = match part_to_cmp.as_str() {
                            "x" => part.x > *target,
                            "m" => part.m > *target,
                            "a" => part.a > *target,
                            "s" => part.s > *target,
                            _ => unreachable!(),
                        };
                        if is_part_valid {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                    }
                    Rule::GoTo { destination } => {
                        queue.push(*workflows_map.get(destination).unwrap());
                        break;
                    }
                }
            }
        }

        unreachable!();
    });

    let part1 = accepted_parts.into_iter().map(|part| part.sum()).sum();

    let accepted_paths = find_all_paths(&workflows_map);

    let ranges = accepted_paths
        .iter()
        .map(|path| get_accepted_part_ranges(&workflows_map, path.clone()))
        .collect_vec();

    let part2 = ranges
        .iter()
        .fold(0, |acc, (x_range, m_range, a_range, s_range)| {
            acc + (x_range.end() - x_range.start() + 1)
                * (m_range.end() - m_range.start() + 1)
                * (a_range.end() - a_range.start() + 1)
                * (s_range.end() - s_range.start() + 1)
        });

    (part1, part2)
}

pub fn main() -> (usize, usize) {
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
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 19114);
        assert_eq!(part2, 167409079868000);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 319062);
        assert_eq!(part2, 118638369682135);
    }
}
