use core::str;
use std::{
    collections::BTreeMap, fmt::{Display, Write}, iter, str::FromStr, time::Instant
};

use aoclib::{input, timing};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> u64 {
    let chunks: Vec<_> = input::empty_line_chunks(txt).collect();
    let wires: Vec<Wire> = chunks[0].lines().map(|l| l.parse().unwrap()).collect();
    let connections: Vec<Connection> = chunks[1].lines().map(|l| l.parse().unwrap()).collect();

    let connection_by_output: HashMap<_, _> =
        connections.into_iter().map(|c| (c.to.clone(), c)).collect();

    let zs: Vec<_> = connection_by_output
        .keys()
        .filter(|k| k.starts_with("z"))
        .cloned()
        .sorted()
        .rev()
        .collect();

    let mut known_wires: HashMap<String, Wire> =
        wires.into_iter().map(|w| (w.name.clone(), w)).collect();

    let mut found_z: Vec<bool> = Vec::with_capacity(zs.len());

    for i in 0..zs.len() {
        let z = &zs[i];
        found_z.push(wire_value(z.clone(), &mut known_wires, &connection_by_output, &mut HashSet::new()).unwrap().value);
    }

    return found_z.iter().fold(0, |acc, &b| (acc << 1) | b as u64);
}

fn part2(txt: &str) -> String {
    let chunks: Vec<_> = input::empty_line_chunks(txt).collect();
    let connections: Vec<Connection> = chunks[1].lines().map(|l| l.parse().unwrap()).collect();

    let initial_connection_by_output: HashMap<_, _> =
        connections.clone()
        .into_iter()
        .map(|c| (c.to.clone(), c))
        .collect();

    let mut connections_by_input: BTreeMap<String, BTreeMap<Gate, String>> = BTreeMap::new();    

    let mut input_xor_output = BTreeMap::new();
    let mut input_and_outputs = BTreeMap::new();
    for c in &connections {

        // map lefts
        if let Some(existing) = connections_by_input.get_mut(&c.from_left) {
            if let Some(out) = existing.insert(c.gate.clone(), c.to.clone()) {
                panic!("replaced existing gate {:?} {:?} to: {:?}", c.gate, c.from_left, out);
            }
        } else {
            connections_by_input.insert(c.from_left.clone(), BTreeMap::from_iter(iter::once((c.gate.clone(), c.to.clone()))));
        }

        // map rights
        if let Some(existing) = connections_by_input.get_mut(&c.from_right) {
            if let Some(out) = existing.insert(c.gate.clone(), c.to.clone()) {
                panic!("replaced existing gate {:?} {:?} to: {:?}", c.gate, c.from_right, out);
            }
        } else {
            connections_by_input.insert(c.from_right.clone(), BTreeMap::from_iter(iter::once((c.gate.clone(), c.to.clone()))));
        }

        if c.from_left.replace("x", "").replace("y", "") == c.from_right.replace("y", "").replace("x", "") {
            let index: u64 = c.from_left.replace("x", "").replace("y", "").parse().unwrap();
            if c.gate == Gate::XOR {
                input_xor_output.insert(index, c.to.clone());
            } else if c.gate == Gate::AND {
                input_and_outputs.insert(index, c.to.clone());
            } else {
                panic!("unexpected OR gate on inputs");
            }
        }
    }


    let mut rule_breaks = HashSet::new();

    for c in &connections {
        if c.from_left == "x00" || c.from_left == "y00" {
            continue;
        }

        if c.to.starts_with("z") && c.to != "z00" {
            if c.to != "z00" {
                if c.from_left.starts_with("x") || c.from_left.starts_with("y") || c.from_right.starts_with("x") || c.from_right.starts_with("y") {
                    println!("[{}] A gate with Zxx as its output cannot directly use Xn or Yn as inputs exept the first bit Z00", &c.to);
                    rule_breaks.insert(c.clone());
                }
            }

            if c.to != "z45" {
                if &c.gate != &Gate::XOR {
                    println!("[{}] Output to a zxx should always be an XOR", c.to);
                    rule_breaks.insert(c.clone());
                }
            }
        }        

        match c.gate {
            Gate::AND => {
                if let Some(input_to) = connections_by_input.get(&c.to) {
                    if input_to.contains_key(&Gate::AND) || input_to.contains_key(&Gate::XOR) {
                        println!("[{}] AND gate can only be input to an OR gate", c.to);
                        rule_breaks.insert(c.clone());
                    }
                }
        
                if let Some(input_left) = initial_connection_by_output.get(&c.from_left) {
                    if input_left.from_left != "x00" && input_left.from_right != "x00" && input_left.gate == Gate::AND {
                        println!("[{}] AND gate cannot take other AND gate as input", input_left.to);
                        rule_breaks.insert(input_left.clone());
                    }
                }
                if let Some(input_right) = initial_connection_by_output.get(&c.from_right) {
                    if input_right.from_left != "x00" && input_right.from_right != "x00" && input_right.gate == Gate::AND {
                        println!("[{}] AND gate cannot take other AND gate as input", input_right.to);
                        rule_breaks.insert(input_right.clone());
                    }
                }
            },
            Gate::XOR => {
                if !c.from_left.starts_with("x") && !c.from_left.starts_with("y") {
                    if !c.to.starts_with("z") {
                        println!("[{}] XOR gate must output to z if non input {} XOR {} -> {}", 
                            c.to, c.from_left, c.from_right, c.to);
                        rule_breaks.insert(c.clone());
                    }                    
                }

                if let Some(input_to) = connections_by_input.get(&c.to) {
                    if input_to.contains_key(&Gate::OR) {
                        println!("[{}] XOR gate can only be input to an AND/XOR gate", c.to);
                        rule_breaks.insert(c.clone());
                    }
                }
                if let Some(input_left) = initial_connection_by_output.get(&c.from_left) {
                    if c.to != "z01" && input_left.gate == Gate::AND {
                        println!("[{}] XOR gate cannot take AND gate as input", input_left.to);
                        rule_breaks.insert(input_left.clone());
                    }
                }
                if let Some(input_right) = initial_connection_by_output.get(&c.from_right) {
                    if c.to != "z01" && input_right.gate == Gate::AND {
                        println!("[{}] XOR gate cannot take AND gate as input", input_right.to);
                        rule_breaks.insert(input_right.clone());
                    }
                }
            },
            Gate::OR => {
                if let Some(input_to) = connections_by_input.get(&c.to) {
                    if input_to.contains_key(&Gate::OR) {
                        println!("[{}] OR gate can only be input of AND/XOR gate", c.to);
                        rule_breaks.insert(c.clone());
                    }
                }
                if let Some(input_left) = initial_connection_by_output.get(&c.from_left) {
                    if input_left.gate != Gate::AND {
                        println!("[{}] OR gate can only take AND gate as input", input_left.to);
                        rule_breaks.insert(input_left.clone());
                    }
                }
                if let Some(input_right) = initial_connection_by_output.get(&c.from_right) {
                    if input_right.gate != Gate::AND {
                        println!("[{}] OR gate can only take AND gate as input", input_right.to);
                        rule_breaks.insert(input_right.clone());
                    }
                }
            },
        }
    }

    rule_breaks.iter().map(|c| &c.to).cloned().sorted().join(",")

}

fn wire_value(
    name: String,
    known_wires: &mut HashMap<String, Wire>,
    connection_by_output: &HashMap<String, Connection>,
    seen: &mut HashSet<String>,
) -> Option<Wire> {
    
    if let Some(wire) = known_wires.get(&name) {
        // println!("wire value: {} = {}", name, wire.value);
        return Some(wire.clone());
    }
    if !seen.insert(name.clone()) {
        return None
    }

    let connection = connection_by_output
        .get(&name)
        .expect(format!("Connection with name: {:?}", name).as_str());
    let left = wire_value(
        connection.from_left.clone(),
        known_wires,
        connection_by_output,
        seen,
    );
    let right = wire_value(
        connection.from_right.clone(),
        known_wires,
        connection_by_output,
        seen
    );
    if left.is_none() || right.is_none() {
        return None;
    }
    let left = left.unwrap();
    let right = right.unwrap();
    let result = match connection.gate {
        Gate::XOR => left.value ^ right.value,
        Gate::OR => left.value || right.value,
        Gate::AND => left.value && right.value,
    };
    let new_wire = Wire {
        name: name.clone(),
        value: result,
    };
    known_wires.insert(name.clone(), new_wire.clone());

    // println!("wire value: {} = {}", name, new_wire.value);
    return Some(new_wire);
}

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Gate {
    XOR,
    OR,
    AND,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Connection {
    from_left: String,
    gate: Gate,
    from_right: String,
    to: String,
}

impl FromStr for Wire {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, value_i) = s.split_once(": ").unwrap();
        let value = match value_i {
            "1" => true,
            _ => false,
        };
        Ok(Self {
            name: name.to_owned(),
            value,
        })
    }
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "XOR" => Ok(Self::XOR),
            "AND" => Ok(Self::AND),
            "OR" => Ok(Self::OR),
            other => Err(format!("bad Gate: {:?}", other)),
        }
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inputs, output) = s.split_once(" -> ").unwrap();
        let inputs: Vec<_> = inputs.split_whitespace().collect();

        Ok(Self {
            from_left: inputs[0].to_owned(),
            gate: inputs[1].parse().expect("gate"),
            from_right: inputs[2].to_owned(),
            to: output.to_owned(),
        })
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
            .and_then(|_| f.write_str(": "))
            .and_then(|_| {
                if self.value {
                    f.write_char('1')
                } else {
                    f.write_char('0')
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1_small() {
        let test_input = include_str!("input.test.small.txt");
        assert_eq!(4, part1(test_input));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2024, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(47666458872582, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!("dnt,gdf,gwc,jst,mcm,z05,z15,z30", part2(test_input));
    }
}
