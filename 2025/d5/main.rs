use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};

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

    fresh.len()
}

fn part2(txt: &str) -> i64 {
    let mut parts = input::empty_line_chunks(txt);
    let ranges: Vec<_> = parts
        .next()
        .expect("line 1")
        .lines()
        .map(|l| l.parse::<FreshRange>().expect("fresh range"))
        .collect();

    let mut len = 1000000;
    let mut result = flatten_all(ranges);
    while len > result.len() {
        len = result.len();
        result = flatten_all(result);
    }
    
    result.into_iter().map(|r| r.len()).sum()
}

fn flatten_all(ranges: Vec<FreshRange>) -> Vec<FreshRange> {
    let mut i = 0;
    let mut result = flatten(i, &ranges);
    while i < result.len() - 1 {
        i += 1;
        result = flatten(i, &result);
    }
    result
}

fn flatten(i: usize, items: &Vec<FreshRange>) -> Vec<FreshRange> {
    let mut current = items[i];
    let mut copy: Vec<FreshRange> = items[i..].iter().cloned().collect();
    let mut result = Vec::new();
    
    while let Some(next) = copy.pop() {
        if let Some(extended) = current.extend(&next) {
            current = extended;
        } else {
            result.push(next);
        }
    }
    result.push(current);
    for prev in &items[0..i] {
        result.push(prev.clone());
    }

    result.reverse();
    result
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

    fn overlap(&self, other: &Self) -> Option<Self> {
        if self.within(other.0) || self.within(other.1) {
            Some(Self(self.0.max(other.0), self.1.min(other.1)))
        } else {
            None
        }        
    }

    fn extend(&self, other: &Self) -> Option<Self> {
        if let Some(FreshRange(_lower_overlap, _upper_overlap)) = self.overlap(&other) {
            Some(FreshRange(self.0.min(other.0), self.1.max(other.1)))
        } else {
            None
        }
    }

    fn len(&self) -> i64 {
        self.1 - self.0 + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::{flatten_all, *};

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
    fn test_overlap() {
        let left = FreshRange(3, 5);
        let right = FreshRange(3, 5);
        assert_eq!(Some(FreshRange(3, 5)), left.overlap(&right));

        let left = FreshRange(4, 5);
        let right = FreshRange(3, 5);
        assert_eq!(Some(FreshRange(4, 5)), left.overlap(&right));

        let left = FreshRange(3, 5);
        let right = FreshRange(3, 4);
        assert_eq!(Some(FreshRange(3, 4)), left.overlap(&right));

        let left = FreshRange(3, 5);
        let right = FreshRange(5, 400);
        assert_eq!(Some(FreshRange(5, 5)), left.overlap(&right));

        let left = FreshRange(16, 20);
        let right = FreshRange(21, 22);
        assert_eq!(None, left.overlap(&right));
    }

    #[test]
    fn test_extend() {
        let left = FreshRange(16, 20);
        let right = FreshRange(12, 18);
        assert_eq!(Some(FreshRange(12, 20)), left.extend(&right));
    }

    #[test]
    fn test_len() {
        assert_eq!(3, FreshRange(3, 5).len());
        assert_eq!(5, FreshRange(10, 14).len());
        assert_eq!(1, FreshRange(10, 10).len());
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(14, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert!(part2(test_input) < 365266687953826);
        assert_eq!(359913027576322, part2(test_input));
    }
}
