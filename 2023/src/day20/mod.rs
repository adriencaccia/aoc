use std::{collections::HashMap, ops::Not};

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Debug, Clone)]
enum Module {
    #[display("broadcaster -> {destinations}")]
    Broadcaster { destinations: String },
    #[display("%{name} -> {destinations}")]
    FlipFlop { name: String, destinations: String },
    #[display("&{name} -> {destinations}")]
    Conjunction { name: String, destinations: String },
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum PulseType {
    Low,
    High,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum ParsedModule {
    Broadcaster {
        destinations: Vec<String>,
    },
    FlipFlop {
        name: String,
        destinations: Vec<String>,
        on: bool,
    },
    Conjunction {
        name: String,
        destinations: Vec<String>,
        pulses_received: HashMap<String, PulseType>,
    },
}

fn parse_destinations(destinations: &str) -> Vec<String> {
    destinations
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

impl From<Module> for ParsedModule {
    fn from(module: Module) -> Self {
        match module {
            Module::Broadcaster { destinations } => ParsedModule::Broadcaster {
                destinations: parse_destinations(&destinations),
            },
            Module::FlipFlop { name, destinations } => ParsedModule::FlipFlop {
                name,
                destinations: parse_destinations(&destinations),
                on: false,
            },
            Module::Conjunction { name, destinations } => ParsedModule::Conjunction {
                name,
                destinations: parse_destinations(&destinations),
                pulses_received: HashMap::new(),
            },
        }
    }
}

fn press_broadcast(modules_map: &mut HashMap<String, ParsedModule>) -> (u32, u32) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut pulses: Vec<(String, PulseType, String)> = vec![(
        "button".to_string(),
        PulseType::Low,
        "broadcaster".to_string(),
    )];

    loop {
        let mut next_pulses: Vec<(String, PulseType, String)> = Vec::new();
        if pulses.is_empty() {
            break;
        }

        while let Some((origin, pulse_type, destination)) = pulses.pop() {
            match pulse_type {
                PulseType::Low => low_pulses += 1,
                PulseType::High => high_pulses += 1,
            }
            let module = modules_map.get_mut(&destination);

            match (module, pulse_type) {
                (None, _) => continue,
                (Some(ParsedModule::Broadcaster { destinations }), _) => {
                    destinations.iter().for_each(|destination| {
                        next_pulses.push((
                            "broadcaster".into(),
                            PulseType::Low,
                            destination.clone(),
                        ));
                    });
                }
                (Some(ParsedModule::FlipFlop { .. }), PulseType::High) => {
                    continue;
                }
                (
                    Some(ParsedModule::FlipFlop {
                        name,
                        destinations,
                        on,
                    }),
                    PulseType::Low,
                ) => {
                    destinations.iter().for_each(|destination| {
                        next_pulses.push((
                            name.clone(),
                            if *on { PulseType::Low } else { PulseType::High },
                            destination.clone(),
                        ));
                    });
                    *on = on.not();
                }
                (
                    Some(ParsedModule::Conjunction {
                        name,
                        destinations,
                        pulses_received,
                    }),
                    pulse_type,
                ) => {
                    pulses_received
                        .entry(origin)
                        .and_modify(|t| *t = pulse_type.clone())
                        .or_insert(pulse_type);
                    let all_high = pulses_received.values().all(|t| t == &PulseType::High);
                    destinations.iter().for_each(|destination| {
                        next_pulses.push((
                            name.clone(),
                            if all_high {
                                PulseType::Low
                            } else {
                                PulseType::High
                            },
                            destination.clone(),
                        ));
                    });
                }
            }
        }
        pulses = next_pulses.clone();
        next_pulses.clear();
    }

    (low_pulses, high_pulses)
}

fn initialize_conjunctions_pulses_received(modules_map: &mut HashMap<String, ParsedModule>) {
    for (name, module) in modules_map.clone().iter() {
        match module {
            ParsedModule::Broadcaster { destinations }
            | ParsedModule::FlipFlop { destinations, .. }
            | ParsedModule::Conjunction { destinations, .. } => {
                destinations.iter().for_each(|destination| {
                    modules_map.entry(destination.clone()).and_modify(|module| {
                        if let ParsedModule::Conjunction {
                            pulses_received, ..
                        } = module
                        {
                            pulses_received.insert(name.clone(), PulseType::Low);
                        }
                    });
                });
            }
        }
    }
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut modules_map: HashMap<String, ParsedModule> = input
        .trim()
        .lines()
        .map(|line| {
            let module: Module = line.parse().unwrap();
            let parsed_module: ParsedModule = module.clone().into();
            match module {
                Module::Broadcaster { .. } => ("broadcaster".to_string(), parsed_module),
                Module::FlipFlop { name, .. } => (name, parsed_module),
                Module::Conjunction { name, .. } => (name, parsed_module),
            }
        })
        .collect();

    initialize_conjunctions_pulses_received(&mut modules_map);

    let (low_pulses, high_pulses) = (0..1000).fold((0, 0), |acc, _| {
        let (low_pulses, high_pulses) = press_broadcast(&mut modules_map);
        (acc.0 + low_pulses, acc.1 + high_pulses)
    });

    let part1 = low_pulses * high_pulses;
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

    const EXAMPLE_INPUT_SIMPLE: &str = indoc! {"
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    "};
    const EXAMPLE_INPUT: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    "};

    #[test]
    fn test_example_simple() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT_SIMPLE);

        assert_eq!(part1, 32000000);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_example() {
        let (part1, part2) = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1, 11687500);
        assert_eq!(part2, 0);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 834323022);
        assert_eq!(part2, 0);
    }
}
