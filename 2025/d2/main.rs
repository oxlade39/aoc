use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::timing;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    txt.split(",")
        .map(|pair| pair.parse::<ProductRange>().unwrap())
        .flat_map(|pr| pr.invalid_ranges())
        .sum()
}

fn part2(txt: &str) -> i64 {
    txt.split(",")
        .map(|r| r.parse::<ProductRange>().unwrap())
        .flat_map(|r| r.0..=r.1)
        .filter(|n| repeats(&format!("{n}")))
        .sum()
}

#[derive(Debug, PartialEq, Hash)]
struct ProductRange(i64, i64);

impl FromStr for ProductRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("-") {
            Some((lower, upper)) => Ok(ProductRange(
                lower.parse().expect("lower"),
                upper.parse().expect("upper"),
            )),
            None => Err(format!("bad input {}", s)),
        }
    }
}

impl ProductRange {
    fn invalid_ranges(&self) -> Vec<i64> {
        let mut invalid = vec![];
        for i in self.0..=self.1 {
            let as_str = format!("{i}");
            if is_symetric_around_half(&as_str, 0) {
                invalid.push(i);
            }
        }
        invalid
    }
}

fn is_symetric_around_half(s: &str, offset: usize) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let half_len = s.len() / 2;

    // got to end
    if offset >= half_len {
        return true;
    }

    let front = &s[offset..offset + 1];
    let back = &s[(half_len + offset)..(half_len + offset + 1)];

    // println!("? {front} == {back} ?");

    if front == back {
        return is_symetric_around_half(s, offset + 1);
    }

    false
}

fn repeats(s: &str) -> bool {
    for size in 0..(s.len() / 2) {
        let seq = &s[0..size + 1];
        if s.len() % (size + 1) != 0 {
            continue;
        }
        let max_num_possible_reps = s.len() / (size + 1);
        let mut offsets = 1..max_num_possible_reps;
        // println!(
        //     "looking for {:?} in offsets for size {} with max reps {}: {:?}",
        //     seq,
        //     size + 1,
        //     max_num_possible_reps,
        //     offsets
        // );

        // let chunks = offsets
        //     .clone()
        //     .map(|offset| (offset * (size + 1))..((offset * (size + 1)) + size + 1))
        //     .collect_vec();
        // println!("chunk ranges for size {}: {:?}", size + 1, chunks);

        // let chunks = offsets
        //     .clone()
        //     .map(|offset| &s[(offset * (size + 1))..((offset * (size + 1)) + size + 1)])
        //     .collect_vec();
        // println!("chunks for size {}: {:?}", size + 1, chunks);

        if offsets
            .any(|offset| &s[(offset * (size + 1))..((offset * (size + 1)) + size + 1)] != seq)
        {
            continue;
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_symetric_around_half() {
        assert_eq!(true, is_symetric_around_half("1010", 0));
        assert_eq!(true, is_symetric_around_half("1188511885", 0));
        assert_eq!(true, is_symetric_around_half("446446", 0));
        assert_eq!(true, is_symetric_around_half("38593859", 0));
    }

    #[test]
    fn test_repeats() {
        assert_eq!(true, repeats("1010"));
        assert_eq!(true, repeats("1188511885"));
        assert_eq!(true, repeats("446446"));
        assert_eq!(true, repeats("38593859"));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(1227775554, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(34826702005, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(4174379265, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(43287141963, part2(test_input));
    }
}
