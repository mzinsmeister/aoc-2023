use std::{fs::read_to_string, collections::{BTreeMap, VecDeque}, borrow::BorrowMut};


#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
enum ModuleState {
    FlipFlop { state: bool },
    Conjunction { states: BTreeMap<String, bool> },
    Broadcast,
    None
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Module {
    name: String,
    state: ModuleState,
    outputs: Vec<String>
}

impl Module {
    fn from_str(input: &str) -> Module {
        let (nametype_str, outputs_str) = input.split_once(" -> ").unwrap();
        let (state, name) = if let Some(name) = nametype_str.strip_prefix("%") {
            (ModuleState::FlipFlop { state: false }, name)
        } else if let Some(name) = nametype_str.strip_prefix("&") {
            (ModuleState::Conjunction { states: BTreeMap::new() }, name)
        } else if nametype_str == "broadcaster" {
            (ModuleState::Broadcast, "broadcaster")
        } else {
            (ModuleState::None, nametype_str)
        };
        let outputs = outputs_str.split(", ").map(|s| s.to_string()).collect();
        Module { name: name.to_string(), state, outputs }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let mut modules: BTreeMap<String, Module> = input_str.lines()
                .filter(|l| !l.is_empty())
                .map(|s| {let m = Module::from_str(s); (m.name.to_owned(), m)})
                .collect();

    let mut inputs = BTreeMap::new();

    for (name, module) in &modules{
        for output in  &module.outputs {
            inputs.entry(output.to_owned()).or_insert(vec![]).push(name.to_owned());
        }
    }

    for module in modules.values_mut() {
        if let ModuleState::Conjunction { states } = &mut module.state {
            for input in &inputs[&module.name] {
                states.insert(input.to_owned(), false);
            }
        }
    }

    let mut modules1 = modules.clone();

    let mut low_count = 0usize;
    let mut high_count = 0usize;

    fn pretty_print_send(name: &str, pulse: bool, target: &str) {
        //let pulse_str = if pulse { "high" } else { "low" };
        //println!("{} -{} -> {}", name, pulse_str, target);
    }

    for _ in 0..1000 {
        let mut simulation_queue: VecDeque<(String, String, bool)> = VecDeque::new();
        simulation_queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while !simulation_queue.is_empty() {
            let (prev, name, pulse) = simulation_queue.pop_front().unwrap();
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }
            if let Some(module) = modules1.get_mut(&name) {
                match &mut module.state {
                    ModuleState::FlipFlop { state: s } => {
                        if !pulse {
                            *s = !*s;
                            for output in &module.outputs {
                                simulation_queue.push_back((module.name.to_owned(), output.to_owned(), *s));
                                pretty_print_send(&module.name, *s, output);
                            }
                        }
                    },
                    ModuleState::Conjunction { states } => {
                        states.insert(prev, pulse);
                        let new_pulse = !states.values().all(|&v| v);
                        for output in &module.outputs {
                            simulation_queue.push_back((module.name.to_owned(), output.to_owned(), new_pulse));
                            pretty_print_send(&module.name, new_pulse, output);
                        }
                    },
                    ModuleState::Broadcast => {
                        for output in &module.outputs {
                            simulation_queue.push_back((module.name.to_owned(), output.to_owned(), pulse));
                            pretty_print_send(&module.name, pulse, output);
                        }
                    },
                    ModuleState::None => {}
                }
            }
        }
    }

    println!("Result 1: {}", low_count * high_count);

    let mut presses: u64 = 0;

    let mut modules2 = modules.clone();

    // Find cycle lengths of rx precursors (only one in my input, guessing this is general rule)
    // Then find lcm of all cycle lengths (this is also only a hacky solution since
    // it could be that during one button press all the inputs are true at some point but never
    // all at the same time, so the lcm would be wrong but it works for my input and guessing 
    // it's true for all inputs)

    let last = modules.iter().find(|m| m.1.outputs.contains(&"rx".to_string())).unwrap().0.to_owned();

    let mut cycle_lengths: BTreeMap<String, u64> = BTreeMap::new();

    'outer:
    loop {
        presses += 1;
        let mut simulation_queue: VecDeque<(String, String, bool)> = VecDeque::new();
        simulation_queue.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while !simulation_queue.is_empty() {
            let (prev, name, pulse) = simulation_queue.pop_front().unwrap();
            if let Some(module) = modules2.get_mut(&name) {
                match &mut module.state {
                    ModuleState::FlipFlop { state: s } => {
                        if !pulse {
                            *s = !*s;
                            for output in &module.outputs {
                                if *s && output.as_str() == last {
                                    if !cycle_lengths.contains_key(output) {
                                        cycle_lengths.insert(name.to_owned(), presses);
                                        if cycle_lengths.len() == 4 {
                                            break 'outer;
                                        }
                                    }
                                }
                                simulation_queue.push_back((module.name.to_owned(), output.to_owned(), *s));
                                pretty_print_send(&module.name, *s, output);
                            }
                        }
                    },
                    ModuleState::Conjunction { states } => {
                        states.insert(prev, pulse);
                        let new_pulse = !states.values().all(|&v| v);
                        for output in &module.outputs {
                            if new_pulse && output.as_str() == last {
                                if !cycle_lengths.contains_key(output) {
                                    cycle_lengths.insert(name.to_owned(), presses);
                                    if cycle_lengths.len() == 4 {
                                        break 'outer;
                                    }
                                }
                            }
                            simulation_queue.push_back((module.name.to_owned(), output.to_owned(), new_pulse));
                            pretty_print_send(&module.name, new_pulse, output);
                        }
                    },
                    ModuleState::Broadcast => {
                        for output in &module.outputs {
                            simulation_queue.push_back((module.name.to_owned(), output.to_owned(), pulse));
                            pretty_print_send(&module.name, pulse, output);
                        }
                    },
                    ModuleState::None => {}
                }
            }
        }
    }

    println!("Result 2: {}", cycle_lengths.values().fold(1, |acc, x| num::integer::lcm(acc, *x)));
}
