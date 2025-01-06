use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

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

fn part1(txt: &str) -> usize {
    let mut all_connections: HashMap<Computer, HashSet<Computer>> = HashMap::new();
    let mut ch_computers: HashSet<Computer> = HashSet::new();

    for Connection([left, right]) in txt.lines().map(|l| l.parse::<Connection>().unwrap()) {
        // left to right
        if let Some(existing) = all_connections.get_mut(&left) {
            existing.insert(right.clone());
        } else {
            all_connections.insert(left.clone(), HashSet::from_iter(vec![right.clone()]));
        }

        // right to left
        if let Some(existing) = all_connections.get_mut(&right) {
            existing.insert(left.clone());
        } else {
            all_connections.insert(right.clone(), HashSet::from_iter(vec![left.clone()]));
        }

        // left may be Chief Historian
        if left.maybe_chief_historian() {
            ch_computers.insert(left);
        }

        // right may be Chief Historian
        if right.maybe_chief_historian() {
            ch_computers.insert(right);
        }
    }

    let mut threes: HashSet<LanParty<3>> = HashSet::new();

    for ch in ch_computers {
        if let Some(connections) = all_connections.get(&ch) {
            for child in connections {
                if let Some(child_connection) = all_connections.get(child) {
                    let inter: HashSet<_> = connections.intersection(child_connection).collect();
                    for item in inter {
                        threes.insert(LanParty::new([
                            ch.clone(),
                            child.clone(),
                            item.clone()
                        ]));
                    }
                }
            }
        }
    }

    threes.len()
}


fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Computer([char; 2]);

impl Computer {
    fn maybe_chief_historian(&self) -> bool {
        self.0[0] == 't'
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Connection([Computer; 2]);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct LanParty<const N: usize>([Computer; N]);

impl<const N: usize> LanParty<N> {
    fn new(mut items: [Computer; N]) -> Self {
        items.sort();
        Self(items)
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        let left = Computer([chars[0], chars[1]]);
        let right = Computer([chars[3], chars[4]]);
        Ok(Connection([left, right]))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(7, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1184, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
