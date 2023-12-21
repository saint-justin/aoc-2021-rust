use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use itertools::Itertools;
use regex::Regex;

/// Day 20, Part 1 -- https://adventofcode.com/2023/day/20
///
/// The elves have all the right pieces, now the parts to fix the machines,
/// now they just need to send the command to boot up all the machines
/// and get that sweet sweet sand flowing once again. The machines are
/// connected via cables which track two distincy types of pulses; High
/// or low pulse types, along with destination modules.
///
/// There are also several different types of module, each one with its
/// own behavior and destination modules.
///
/// Flip-Flop modules (%) are either on or off, starting in the off state.
/// If it receives a high pulse, it's ignored and nothing happens. If it
/// receives a low pulse, it flips its state and sends a pulse to its
/// destinations. If the low pulse was received while the flip-flip was on,
/// then it turns off and sends a sends a low pulse. If it was off, it sends
/// a high pulse and turns on.
///
/// Conjunction modules (&) remember the most recent pulse passed from EACH
/// of its connected input modules (defaulting to low pulses before any
/// pulses are received). When it receives a pulse, it will replace the
/// previous pulse with the new pulse in its store. Then, if EVERY input
/// remembered is a high pulse, it sends a low pulse. Otherwise, it sends a
/// high pulse.
///
/// The Broadcaster module (broadcaster) that just forwards whatever pulse it
/// gets to all destination modules.
///
/// The Broadcaster Button is a button in elf HQ that will send a single low
/// pulse to the Broadcaster module every time it's pressed.
///
/// broadcaster -> a
/// %a -> inv, con
/// &inv -> b
/// %b -> con
/// &con -> output
///
/// The input is a series of modules with their type, name, and outputs.
/// Calculate how many low pulses and high pulses reach the output after
/// pressing the button 1000 times. What is their product?
pub fn find_frequency_product(input_modules: &Vec<&str>) -> u64 {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut conjunctions = HashMap::new();
    let re = Regex::new(r"([a-zA-Z]{1,})").unwrap();

    // first time through, just collect the conjunction modules
    for module in input_modules {
        if module.chars().nth(0).unwrap() == '&' {
            let (id, listeners) = get_input_parts(module, &re);
            conjunctions.insert(id, Conjunction::new(&listeners));
        }
    }

    // second time through, collect broadcast + flipflips and fill conjunction inputs
    for module in input_modules {
        let (id, listeners) = get_input_parts(module, &re);
        match module.chars().nth(0).unwrap() {
            '&' => {} // account for but skip
            '%' => {
                modules.insert(id.clone(), Module::FlipFlop(FlipFlop::new(&listeners)));
                for listener in &listeners {
                    if let Some(conjunction) = conjunctions.get_mut(listener) {
                        conjunction.add_listener(id.clone());
                    }
                }
            }
            'b' => {
                modules.insert(
                    id.clone(),
                    Module::Broadcaster(Broadcaster::new(&listeners)),
                );
                for listener in &listeners {
                    if let Some(conjunction) = conjunctions.get_mut(listener) {
                        conjunction.add_listener(id.clone());
                    }
                }
            }

            _ => panic!("Module type unaccounted for: {}", module),
        }
    }

    for (id, conjunction) in conjunctions {
        modules.insert(id, Module::Conjunction(conjunction));
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for i in 0..1000 {
        if let Some((low, high)) = press_button(&mut modules) {
            low_pulses += low;
            high_pulses += high;
        }
    }

    low_pulses * high_pulses
}

fn press_button(modules: &mut HashMap<String, Module>) -> Option<(u64, u64)> {
    let mut to_explore: VecDeque<(Pulse, String, String)> = VecDeque::new();
    to_explore.push_back((Pulse::LOW, "broadcaster".to_owned(), "button".to_owned()));
    let mut low_outputs = 0_u64;
    let mut high_outputs = 0_u64;
    while to_explore.len() > 0 {
        let (pulse, target, sender) = to_explore.pop_front().unwrap();
        match pulse {
            Pulse::LOW => low_outputs += 1,
            Pulse::HIGH => high_outputs += 1,
        }

        let module = modules.get_mut(&target);
        if module.is_none() {
            continue;
        }

        if let Some((pulse, targets)) = module.unwrap().process_pulse(pulse, sender) {
            for t in targets {
                if t == "output" {
                    match pulse {
                        Pulse::LOW => low_outputs += 1,
                        Pulse::HIGH => high_outputs += 1,
                    }

                    if t == "rx" && pulse == Pulse::LOW {
                        return None;
                    }
                } else {
                    to_explore.push_back((pulse, t, target.clone()));
                }
            }
        }
    }

    Some((low_outputs, high_outputs))
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

impl Module {
    pub fn process_pulse(&mut self, pulse: Pulse, sender: String) -> Option<(Pulse, Vec<String>)> {
        // println!("Module State: {:?}", self);
        match self {
            Module::FlipFlop(module) => module.process_pulse(pulse, sender),
            Module::Conjunction(module) => module.process_pulse(pulse, sender),
            Module::Broadcaster(module) => module.process_pulse(pulse, sender),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    HIGH,
    LOW,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    on: bool,
    listeners: Vec<String>,
}

impl FlipFlop {
    pub fn new(listener_ids: &Vec<String>) -> FlipFlop {
        FlipFlop {
            on: false,
            listeners: listener_ids.clone(),
        }
    }

    fn process_pulse(&mut self, pulse: Pulse, _sender: String) -> Option<(Pulse, Vec<String>)> {
        match pulse {
            Pulse::HIGH => return None,
            Pulse::LOW => {
                self.on = !self.on;
                if self.on {
                    return Some((Pulse::HIGH, self.listeners.clone()));
                } else {
                    return Some((Pulse::LOW, self.listeners.clone()));
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjunction {
    inputs: HashMap<String, Pulse>,
    listeners: Vec<String>,
}

impl Conjunction {
    pub fn new(listener_ids: &Vec<String>) -> Conjunction {
        return Conjunction {
            inputs: HashMap::new(),
            listeners: listener_ids.clone(),
        };
    }

    pub fn add_listener(&mut self, sender_id: String) {
        self.inputs.insert(sender_id, Pulse::LOW);
    }

    fn process_pulse(&mut self, pulse: Pulse, sender: String) -> Option<(Pulse, Vec<String>)> {
        self.inputs.insert(sender, pulse);
        match self
            .inputs
            .iter()
            .all(|(_name, pulse)| pulse == &Pulse::HIGH)
        {
            true => Some((Pulse::LOW, self.listeners.clone())),
            false => Some((Pulse::HIGH, self.listeners.clone())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Broadcaster {
    listeners: Vec<String>,
}

impl Broadcaster {
    pub fn new(listener_ids: &Vec<String>) -> Broadcaster {
        return Broadcaster {
            listeners: listener_ids.clone(),
        };
    }

    fn process_pulse(&mut self, pulse: Pulse, _sender: String) -> Option<(Pulse, Vec<String>)> {
        return Some((pulse, self.listeners.clone()));
    }
}

// Helpers
fn get_input_parts(input: &str, re: &Regex) -> (String, Vec<String>) {
    let parts = re
        .find_iter(input)
        .map(|m| m.as_str().to_owned())
        .collect_vec();

    (parts[0].clone(), parts[1..].to_vec())
}
