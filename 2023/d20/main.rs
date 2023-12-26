use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    time::Instant,
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let mut module_config: ModuleConfig = txt.parse().expect("valid module config");

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    for _ in 0..1000 {
        let (low, high) = module_config.push_button();
        low_pulse_count += low;
        high_pulse_count += high;
    }

    high_pulse_count * low_pulse_count
}

fn part2(txt: &str) -> usize {
    0
}

const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";

struct ModuleConfig {
    modules: HashMap<String, Module>,
}

impl ModuleConfig {
    fn push_button(&mut self) -> (usize, usize) {
        let mut low_pulse_count = 0;
        let mut high_pulse_count = 0;

        let mut inbox: VecDeque<(String, String, Pulse)> = VecDeque::new();
        inbox.push_back((BUTTON.to_owned(), BUTTON.to_owned(), Pulse::Low));

        while let Some((from_module, to_module, pulse)) = inbox.pop_front() {
            match pulse {
                Pulse::High => {
                    high_pulse_count += 1;
                }
                Pulse::Low => {
                    low_pulse_count += 1;
                }
            }

            // why do I have a module that doesn't exist? seems wrong - or at least unspecified
            if let Some(module) = self.modules.get_mut(&to_module) {
                match module {
                    Module::Button => {
                        inbox.push_back((BUTTON.to_owned(), BROADCASTER.to_owned(), pulse));
                    }
                    Module::Broadcaster(children) => {
                        for c in children.iter() {
                            inbox.push_back((BROADCASTER.to_owned(), c.clone(), pulse));
                        }
                    }
                    Module::FlipFlop(name, ref mut state, children) => {
                        // Flip-flop modules (prefix %) are either on or off;
                        // they are initially off.
                        // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
                        // However, if a flip-flop module receives a low pulse, it flips between on and off.
                        // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
                        if pulse == Pulse::Low {
                            let pulse = state.flip();
                            for c in children.iter() {
                                inbox.push_back((name.to_owned(), c.clone(), pulse.clone()));
                            }
                        }
                    }
                    Module::Conjunction(name, inputs, children) => {
                        // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules;
                        // they initially default to remembering a low pulse for each input. When a pulse is received,
                        // the conjunction module first updates its memory for that input.
                        // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
                        inputs.insert(from_module, pulse);
                        let to_send = if inputs.values().all(|p| p == &Pulse::High) {
                            Pulse::Low
                        } else {
                            Pulse::High
                        };
                        for c in children.iter() {
                            inbox.push_back((name.clone(), c.clone(), to_send));
                        }
                    }
                }
            }
        }
        (low_pulse_count - 1, high_pulse_count)
    }
}

enum Module {
    Button,
    Broadcaster(Vec<String>),
    FlipFlop(String, FlipFlopState, Vec<String>),
    Conjunction(String, HashMap<String, Pulse>, Vec<String>),
}

impl Module {
    fn name(&self) -> &str {
        match self {
            Module::Button => BUTTON,
            Module::Broadcaster(_) => BROADCASTER,
            Module::FlipFlop(name, _, _) => &name,
            Module::Conjunction(name, _, _) => &name,
        }
    }

    fn children(&self) -> Vec<String> {
        match self {
            Module::Button => vec![BROADCASTER.to_owned()],
            Module::Broadcaster(children) => children.clone(),
            Module::FlipFlop(_, _, children) => children.clone(),
            Module::Conjunction(_, _, children) => children.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlipFlopState {
    On,
    Off,
}

impl FlipFlopState {
    /// if a flip-flop module receives a low pulse, it flips between on and off.
    /// If it was off, it turns on and sends a high pulse.
    /// If it was on, it turns off and sends a low pulse.
    fn flip(&mut self) -> Pulse {
        let (next, pulse) = match self {
            FlipFlopState::Off => (Self::On, Pulse::High),
            FlipFlopState::On => (Self::Off, Pulse::Low),
        };
        *self = next;
        pulse
    }
}

impl FromStr for ModuleConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules: HashMap<String, Module> = HashMap::new();
        modules.insert(BUTTON.to_owned(), Module::Button);

        let mut module_children: HashMap<String, Vec<String>> = HashMap::new();

        let modules_list: Vec<_> = s.lines().map(|l| l.parse::<Module>().unwrap()).collect();

        for m in modules_list {
            module_children.insert(m.name().to_owned(), m.children());
            modules.insert(m.name().to_owned(), m);
        }

        // connect up the conjunction inputs
        // and set their initial pulse to Low
        for (parent, v) in module_children {
            for child_name in v {
                if let Some(child) = modules.get_mut(&child_name) {
                    match child {
                        Module::Conjunction(_, inputs, _) => {
                            inputs.insert(parent.clone(), Pulse::Low);
                        }
                        _ => { /* noop */ }
                    }
                }
            }
        }

        Ok(Self { modules })
    }
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('%') => {
                let rest: Vec<_> = s[1..].split(" -> ").collect();
                let name = rest[0].to_owned();
                let connections: Vec<_> = rest[1].split(", ").map(&str::to_owned).collect();

                Ok(Module::FlipFlop(name, FlipFlopState::Off, connections))
            }
            Some('&') => {
                let rest: Vec<_> = s[1..].split(" -> ").collect();
                let name = rest[0].to_owned();
                let connections: Vec<_> = rest[1].split(", ").map(&str::to_owned).collect();

                Ok(Module::Conjunction(name, HashMap::new(), connections))
            }
            Some('b') => {
                let left_right: Vec<_> = s.split(" -> ").collect();
                let connections: Vec<_> = left_right[1].split(", ").map(&str::to_owned).collect();
                Ok(Module::Broadcaster(connections))
            }
            _ => panic!("bad module"),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(32000000, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p1_2() {
        assert_eq!(11687500, part1(include_str!("input.test2.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_pt1_step() {
        let txt = include_str!("input.test.txt");
        let mut module_config: ModuleConfig = txt.parse().expect("valid module config");
        let (low, high) = module_config.push_button();
        assert_eq!(8, low);
        assert_eq!(4, high);
    }
}
