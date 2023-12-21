use std::{cell::RefCell, collections::HashMap, ops::Not, rc::Rc};

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

fn press_broadcast(
    modules_map: &mut HashMap<String, ParsedModule>,
    idx: usize,
    mut origins: Option<&mut HashMap<String, Option<usize>>>,
    rx_origin: Option<&str>,
) -> (usize, usize) {
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
            if let Some(rx_origin) = rx_origin {
                if pulse_type == PulseType::High && destination == rx_origin {
                    origins
                        .as_mut()
                        .unwrap()
                        .entry(origin.clone())
                        .and_modify(|v| *v = Some(idx))
                        .or_insert(Some(idx));
                }
            }

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

fn parse_input(input: &str) -> HashMap<String, ParsedModule> {
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
    modules_map
}

fn part1(input: &str) -> usize {
    let mut modules_map = parse_input(input);

    let (low_pulses, high_pulses) = (1..=1000).fold((0, 0), |acc, idx| {
        let (low_pulses, high_pulses) = press_broadcast(&mut modules_map, idx, None, None);
        (acc.0 + low_pulses, acc.1 + high_pulses)
    });

    low_pulses * high_pulses
}

fn part2(input: &str) -> usize {
    let mut modules_map = parse_input(input);

    let rx_origin = modules_map
        .values()
        .find_map(|module| match module {
            // module before rx is always going to be a Conjunction
            ParsedModule::Conjunction {
                name, destinations, ..
            } => {
                if destinations.contains(&"rx".to_string()) {
                    return Some(name.clone());
                }
                None
            }
            _ => None,
        })
        .unwrap();

    let origins: HashMap<String, Option<usize>> = modules_map
        .values()
        .filter_map(|module| match module {
            ParsedModule::Conjunction {
                name, destinations, ..
            }
            | ParsedModule::FlipFlop {
                name, destinations, ..
            } => {
                if destinations.contains(&rx_origin) {
                    return Some((name.clone(), None));
                }
                None
            }
            _ => None,
        })
        .collect();

    let mut idx = 0;
    let origins = Rc::new(RefCell::new(origins));

    loop {
        idx += 1;
        let origins_clone = Rc::clone(&origins);
        press_broadcast(
            &mut modules_map,
            idx,
            Some(&mut origins_clone.borrow_mut()),
            Some(&rx_origin),
        );

        if origins.borrow().values().all(|v| v.is_some()) {
            break;
        }
    }

    let value = origins.borrow().values().map(|v| v.unwrap()).product();
    value
}

pub fn main() -> (usize, usize) {
    let input = include_str!("input.txt");
    let (part1, part2) = (part1(input), part2(input));
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
        let part1 = part1(EXAMPLE_INPUT_SIMPLE);

        assert_eq!(part1, 32000000);
    }

    #[test]
    fn test_example() {
        let part1 = part1(EXAMPLE_INPUT);

        assert_eq!(part1, 11687500);
    }

    #[test]
    fn test_main() {
        let (part1, part2) = main();

        assert_eq!(part1, 834323022);
        assert_eq!(part2, 225386464601017);
    }
}
