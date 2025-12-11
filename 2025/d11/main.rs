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

fn part1(txt: &str) -> i64 {
    let connections: HashMap<String, Vec<String>> = txt.lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let key = parts.next().expect("key");
            let key = &key[0..key.len()-1];
            let values: Vec<_> = parts.map(|s| s.to_owned()).collect();
            (key.to_owned(), values)
        })
        .collect();

    // println!("connections: {:?}", connections);

    count("you".to_owned(), &connections)
}

fn count(
    current: String, 
    connections: &HashMap<String, Vec<String>>,
) -> i64 {
    if current == "out" {
        return 1
    }
    if let Some(next) = connections.get(&current) {
        let mut sum = 0;
        for n in next {
            sum += count(n.clone(), connections);
        }
        sum

    } else {
        0
    }
}

fn part2(txt: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(5, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(413, part1(test_input));
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
