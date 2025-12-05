use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let mut parts = input::empty_line_chunks(txt);
    let ranges: Vec<_> = parts
        .next()
        .expect("line 1")
        .lines()
        .map(|l| l.parse::<FreshRange>().expect("fresh range"))
        .collect();

    let fresh: Vec<_> = parts
        .next()
        .expect("ingredients")
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .filter(|i| ranges.iter().any(|r| r.within(*i)))
        .collect();

    println!("fresh: {:?}", fresh);

    fresh.len()
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FreshRange(i64, i64);

impl FromStr for FreshRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, upper) = s.split_once("-").unwrap();
        Ok(FreshRange(lower.parse().unwrap(), upper.parse().unwrap()))
    }
}

impl FreshRange {
    fn within(&self, ingredient: i64) -> bool {
        ingredient >= self.0 && ingredient <= self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(3, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(840, part1(test_input));
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
