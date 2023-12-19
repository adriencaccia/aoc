use std::collections::HashMap;

use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Debug)]
enum Rule {
    #[display("{part_to_cmp}<{target}:{destination}")]
    LessThan {
        part_to_cmp: String,
        target: u32,
        destination: String,
    },
    #[display("{part_to_cmp}>{target}:{destination}")]
    GreaterThan {
        part_to_cmp: String,
        target: u32,
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
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn parse_input(input: &str) -> (u32, u32) {
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
                        if *part_to_cmp == 'x'.to_string() && part.x < *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 'm'.to_string() && part.m < *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 'a'.to_string() && part.a < *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 's'.to_string() && part.s < *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                    }
                    Rule::GreaterThan {
                        part_to_cmp,
                        target,
                        destination,
                    } => {
                        if *part_to_cmp == 'x'.to_string() && part.x > *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 'm'.to_string() && part.m > *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 'a'.to_string() && part.a > *target {
                            queue.push(*workflows_map.get(destination).unwrap());
                            break;
                        }
                        if *part_to_cmp == 's'.to_string() && part.s > *target {
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

    let part1 = accepted_parts
        .into_iter()
        .map(|part| part.sum())
        .sum::<u32>();
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
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 319062);
        assert_eq!(part2, 0);
    }
}
