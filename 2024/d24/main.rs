use core::str;
use std::{collections::BTreeMap, fmt::{Display, Write}, i64, str::FromStr, time::Instant, usize};

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

    let connection_by_output: HashMap<_, _> = connections.into_iter()
        .map(|c| (c.to.clone(), c))
        .collect();

    let mut zs: Vec<_> = connection_by_output.keys()
        .filter(|k| k.starts_with("z"))
        .cloned()
        .sorted()
        .rev()
        .collect();

    let mut known_wires: HashMap<String, Wire> = wires.into_iter()
        .map(|w| (w.name.clone(), w))
        .collect();

    let mut found_z: Vec<bool> = Vec::with_capacity(zs.len());

    for i in 0..zs.len() {
        let z = &zs[i];
        wire_value(
            z.clone(),
            &mut known_wires, 
            &connection_by_output
        );
        let wire= known_wires.get(z).unwrap();
        found_z.push(wire.value);
    }

    // for (k, v) in BTreeMap::from_iter(known_wires.iter()) {
    //     println!("{}", v);
    // }

    // println!("z: {:?}", found_z);

    return found_z.iter().fold(0, |acc, &b| (acc << 1) | b as u64);
}

fn part2(txt: &str) -> i64 {
    0
}

fn wire_value(
    name: String,
    known_wires: &mut HashMap<String, Wire>,
    connection_by_output: &HashMap<String, Connection>,
) -> Wire {
    if let Some(wire) = known_wires.get(&name) {
        return wire.clone();
    }

    let connection = connection_by_output.get(&name).expect(format!("Connection with name: {:?}", name).as_str());
    let left = wire_value(connection.from_left.clone(), known_wires, connection_by_output);
    let right = wire_value(connection.from_right.clone(), known_wires, connection_by_output);
    let result = match connection.gate {
        Gate::XOR => left.value ^ right.value,
        Gate::OR => left.value || right.value,
        Gate::AND => left.value && right.value,
    };
    let new_wire = Wire { name: name.clone(), value: result };
    known_wires.insert(name, new_wire.clone());

    return new_wire;
}

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: bool,
}

#[derive(Debug, Clone)]
enum Gate {
    XOR,
    OR,
    AND,
}

#[derive(Debug, Clone)]
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
        f.write_str(&self.name).and_then(|_| f.write_str(": ")).and_then(|_| {
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
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
