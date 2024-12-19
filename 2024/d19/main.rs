use core::str;
use std::{i64, time::Instant, usize};

use aoclib::{input, timing};
use hashbrown::HashMap;
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
    let towels = towels.split(", ").map(|p| TowelPattern(p));
    
    let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();

    for t in towels {
        let fc = t.0.chars().next().unwrap();
        if let Some(existing) = patterns.get_mut(&fc) {
            existing.push(t);
        } else {
            let mut bh = Vec::new();
            bh.push(t);
            patterns.insert(fc, bh);
        }
    }

    let mut c = 0;
    let to_check: Vec<_> = to_check.lines().collect();

    let mut memo = HashMap::with_capacity(20000);
    for item in to_check {
        if count(item, 0, &mut patterns, &mut memo) > 0 {
            c += 1;
        }
    }
    println!("size: {}", memo.len());
    c
}

fn part2(txt: &str) -> usize {
    let (towels, to_check) = input::empty_line_chunks(txt).tuples().next().unwrap();
    let towels = towels.split(", ").map(|p| TowelPattern(p));
    
    let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();

    for t in towels {
        let fc = t.0.chars().next().unwrap();
        if let Some(existing) = patterns.get_mut(&fc) {
            existing.push(t);
        } else {
            let mut bh = Vec::new();
            bh.push(t);
            patterns.insert(fc, bh);
        }
    }

    let mut c = 0;
    let to_check: Vec<_> = to_check.lines().collect();
    let mut memo = HashMap::with_capacity(20000);
    for item in to_check {
        c += count(item, 0, &patterns, &mut memo);
    }
    c
}

fn count<'a>(
    s: &'a str,
    position: usize,
    patterns: &HashMap<char, Vec<TowelPattern>>,
    memo: &mut HashMap<&'a str, usize>,
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
        memo.insert(remainder, combos);
        return combos;
    }
    0
}

#[derive(Debug)]
struct TowelPattern<'a>(&'a str);

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_find() {
        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![TowelPattern("r")]);
        assert_eq!(true, count("r", 0, &mut patterns, &mut HashMap::new()) > 0);

        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![TowelPattern("rw")]);
        assert_eq!(true, count("rw", 0, &mut patterns, &mut HashMap::new()) > 0);

        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![
            TowelPattern("r"),
            TowelPattern("rw"),
        ]);
        assert_eq!(true, count("rw", 0, &mut patterns, &mut HashMap::new()) > 0);
        assert_eq!(true, count("rrw", 0, &mut patterns, &mut HashMap::new()) > 0);

    }

    #[test]
    fn test_find_greedy() {
        // greedy
        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![
            TowelPattern("rw"),
            TowelPattern("rww"),            
        ]);
        patterns.insert('w', vec![
            TowelPattern("wr"),
        ]);
        assert_eq!(true, count("rwwr", 0, &mut patterns, &mut HashMap::new()) > 0);
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
        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![TowelPattern("r")]);
        assert_eq!(1, count("r", 0, &mut patterns, &mut HashMap::new()));

        let mut patterns: HashMap<char, Vec<TowelPattern>> = HashMap::new();
        patterns.insert('r', vec![
            TowelPattern("r"),
            TowelPattern("rw"),
        ]);
        patterns.insert('w', vec![
            TowelPattern("w"),
        ]);
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
