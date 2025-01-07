use core::str;
use std::{
    fmt::{Display, Write},
    hash::Hash,
    str::FromStr,
    time::Instant,
    usize,
};

use aoclib::timing;
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
                        threes.insert(LanParty::new([ch.clone(), child.clone(), item.clone()]));
                    }
                }
            }
        }
    }

    threes.len()
}

fn part2(txt: &str) -> String {
    let mut all_connections: HashMap<Computer, HashSet<Computer>> = HashMap::new();

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
    }

    largest_clique(&all_connections)
        .into_iter()
        .sorted()
        .join(",")
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Computer([char; 2]);

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.0[0])
            .and_then(|_| f.write_char(self.0[1]))
    }
}

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

fn largest_clique<T>(graph: &HashMap<T, HashSet<T>>) -> HashSet<T>
where
    T: Clone + Hash + Eq,
{
    let mut cliques = Vec::new();
    let mut key_set: HashSet<T> = graph.keys().cloned().collect();
    bron_kerbosch(
        graph,
        &HashSet::new(),
        &mut key_set,
        HashSet::new(),
        &mut cliques,
    );

    cliques.sort_by_key(|clique| clique.len());
    cliques.pop().unwrap()
}

/// The bron_kerbosch algo, for finding all maximal cliques
/// See https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
/// and https://en.wikipedia.org/wiki/Clique_(graph_theory)
fn bron_kerbosch<T>(
    graph: &HashMap<T, HashSet<T>>,
    r: &HashSet<T>,
    p: &mut HashSet<T>,
    x: HashSet<T>,
    cliques: &mut Vec<HashSet<T>>,
) where
    T: Clone + Hash + Eq,
{
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }

    while let Some(v) = p.iter().next().cloned().and_then(|item| p.take(&item)) {
        let union: HashSet<_> = r
            .union(&HashSet::from_iter(vec![v.clone()]))
            .cloned()
            .collect();
        let v_connections = graph.get(&v).unwrap();
        let mut p_inter: HashSet<_> = p.intersection(v_connections).cloned().collect();
        let x_inter: HashSet<_> = x.intersection(v_connections).cloned().collect();
        bron_kerbosch(graph, &union, &mut p_inter, x_inter, cliques);
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
        assert_eq!("co,de,ka,ta", part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!("hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv", part2(test_input));
    }
}
