use core::str;
use std::{cmp::Reverse, fmt::format, i64, ops::Index, str::FromStr, time::Instant, usize};

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
    txt.lines()
        .map(|l| l.parse::<Bank>().expect("Bank"))
        .map(|b| b.joltage())
        .sum()
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct Bank(Vec<Batery>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
struct Batery(usize);

impl FromStr for Batery {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<usize>() {
            Ok(i) => Ok(Batery(i)),
            Err(e) => Err(format!("bad input '{}'", s)),
        }
    }
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bats = s
            .split("")
            .filter(|s| &"" != s)
            .map(|b| b.parse::<Batery>().expect("battery"))
            .collect();
        return Ok(Bank(bats));
    }
}

impl Bank {
    fn joltage(&self) -> usize {
        let mut positions: [usize; 10] = [0; 10];
        let ordered: Vec<_> = self.0.iter().sorted_by(|a, b| b.cmp(a)).collect();
        for i in 0..self.0.len() {
            let value = &self.0[i];
            let current = positions[value.0];
            if current == 0 {
                positions[value.0] = i;
            }                        
        }

        // max is last
        let tens = if positions[ordered[0].0] == self.0.len() - 1 {
            ordered[1]
        } else {
            ordered[0]
        };

        println!("tens: {:?} @ {} of {}", tens, positions[tens.0], self.0.len());
        println!("positions: {:?}", positions);

        let mut units = 0;
        for i in (positions[tens.0] + 1)..self.0.len() {
            units = units.max(self.0[i].0);
        }

        tens.0 * 10 + units
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_bank_parse() {
        let s = "987654321111111";
        let b = s.parse::<Bank>();
        assert_eq!(Ok(Bank(vec![
            Batery(9),
            Batery(8),
            Batery(7),
            Batery(6),
            Batery(5),
            Batery(4),
            Batery(3),
            Batery(2),
            Batery(1),
            Batery(1),
            Batery(1),
            Batery(1),
            Batery(1),
            Batery(1),
            Batery(1),
        ])), b);
    }

    #[test]
    fn test_bank_joltage() {
        let s = "987654321111111";
        let b = s.parse::<Bank>().unwrap();
        assert_eq!(98, b.joltage());
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(357, part1(test_input));
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
