use core::str;
use std::{collections::BinaryHeap, i64, str::FromStr, time::Instant, usize};

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

fn part1(txt: &str) -> i64 {
    let (towels, to_check) = input::empty_line_chunks(txt).tuples().next().unwrap();
    let towels = towels.split(", ").map(|p| TowelPattern(p.to_owned()));
    
    let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();

    for t in towels {
        let fc = t.0.chars().next().unwrap();
        if let Some(existing) = patterns.get_mut(&fc) {
            existing.push(t);
        } else {
            let mut bh = BinaryHeap::new();
            bh.push(t);
            patterns.insert(fc, bh);
        }
    }

    let mut count = 0;
    let to_check: Vec<_> = to_check.lines().collect();

    for item in to_check {
        if find(item, 0, &mut patterns, &mut HashSet::new()) {
            count += 1;
        }
    }
    count
}

fn part2(txt: &str) -> usize {
    let (towels, to_check) = input::empty_line_chunks(txt).tuples().next().unwrap();
    let towels = towels.split(", ").map(|p| TowelPattern(p.to_owned()));
    
    let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();

    for t in towels {
        let fc = t.0.chars().next().unwrap();
        if let Some(existing) = patterns.get_mut(&fc) {
            existing.push(t);
        } else {
            let mut bh = BinaryHeap::new();
            bh.push(t);
            patterns.insert(fc, bh);
        }
    }

    let mut c = 0;
    let to_check: Vec<_> = to_check.lines().collect();

    for item in to_check {
        c += count(item, 0, &patterns, &mut HashMap::new());
    }
    c
}

fn find(
    s: &str,
    position: usize,
    patterns: &mut HashMap<char, BinaryHeap<TowelPattern>>,
    seen: &mut HashSet<String>,
) -> bool {
    let sub = &s[position..];
    if sub.is_empty() {
        return true;
    }
    if seen.contains(sub) {
        return false;
    }
    let c = sub.chars().next().expect(&format!("no char in {:?}", sub));
    if let Some(potential_children) = patterns.get_mut(&c) {

        let mut clone = potential_children.clone();
        while let Some(next) = clone.pop() {
            if next.0.len() > sub.len() {
                continue;
            }

            let sub_sub = &sub[0..next.0.len()];
            if sub_sub == next.0 {
                // try match recursive
                let matched_pos = position + next.0.len();
                let _rhs = &s[matched_pos..];
                if find(s, matched_pos, patterns, seen) {                    
                    seen.insert(sub_sub.to_owned());
                    return true;
                }
            }
        }
    }
    seen.insert(sub.to_owned());
    false
}

fn count(
    s: &str,
    position: usize,
    patterns: &HashMap<char, BinaryHeap<TowelPattern>>,
    memo: &mut HashMap<String, usize>,
) -> usize {

    if position == s.len() {
        return 1;
    }

    let remainder = &s[position..];
    if remainder.is_empty() {
        return 1;
    }

    if let Some(exising) = memo.get(remainder) {
        return *exising;
    }

    let next_char = remainder.chars().next().expect("non empty string");

    if let Some(mapping) = patterns.get(&next_char) {        
        let mut combos = 0;
        for child in mapping {
            if child.0.len() > remainder.len() {
                // can't be match at too short
                continue;
            }
            let begginning = &remainder[0..child.0.len()];
            if begginning == child.0 {
                combos += count(s, position + child.0.len(), patterns, memo);
            }            
        }
        memo.insert(remainder.to_owned(), combos);
        return combos;
    }
    0
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct TowelPattern(String);

impl PartialOrd for TowelPattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_len = self.0.len();
        let other_len = other.0.len();
        my_len.partial_cmp(&other_len)
    }
}

impl Ord for TowelPattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_len = self.0.len();
        let other_len = other.0.len();
        my_len.cmp(&other_len)
    }
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_find() {
        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![TowelPattern("r".to_owned())]));
        assert_eq!(true, find("r", 0, &mut patterns, &mut HashSet::new()));

        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![TowelPattern("rw".to_owned())]));
        assert_eq!(true, find("rw", 0, &mut patterns, &mut HashSet::new()));

        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![
            TowelPattern("r".to_owned()),
            TowelPattern("rw".to_owned()),
        ]));
        assert_eq!(true, find("rw", 0, &mut patterns, &mut HashSet::new()));
        assert_eq!(true, find("rrw", 0, &mut patterns, &mut HashSet::new()));

    }

    #[test]
    fn test_find_greedy() {
        // greedy
        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![
            TowelPattern("rw".to_owned()),
            TowelPattern("rww".to_owned()),            
        ]));
        patterns.insert('w', BinaryHeap::from_iter(vec![
            TowelPattern("wr".to_owned()),
        ]));
        assert_eq!(true, find("rwwr", 0, &mut patterns, &mut HashSet::new()));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(6, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(344, part1(test_input));
    }

    #[test]
    fn test_count() {
        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![TowelPattern("r".to_owned())]));
        assert_eq!(1, count("r", 0, &mut patterns, &mut HashMap::new()));

        let mut patterns: HashMap<char, BinaryHeap<TowelPattern>> = HashMap::new();
        patterns.insert('r', BinaryHeap::from_iter(vec![
            TowelPattern("r".to_owned()),
            TowelPattern("rw".to_owned()),
        ]));
        patterns.insert('w', BinaryHeap::from_iter(vec![
            TowelPattern("w".to_owned()),
        ]));
        assert_eq!(2, count("rw", 0, &mut patterns, &mut HashMap::new()));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(16, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(996172272010026, part2(test_input));
    }
}
