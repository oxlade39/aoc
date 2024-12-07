use core::str;
use std::{collections::HashSet, str::FromStr, time::Instant};

use aoclib::grid::{FromChar, Grid, GridPosition};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{:.2}s", now.elapsed().as_secs_f64());
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.parse::<Calibration>().unwrap())
        .filter(|calibration| calibration.can_work())
        .map(|c| c.total)
        .sum()
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone)]
struct Calibration {
    total: i64,
    values: Vec<i64>,
}

impl FromStr for Calibration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let total = left.parse().unwrap();
        let values = right
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        Ok(Calibration { total, values })
    }
}

impl Calibration {
    fn can_work(&self) -> bool {
        self.can_work_n(0, 0)
    }

    fn can_work_n(&self, running_total: i64, pos: usize) -> bool {
        if pos == self.values.len() {
            return running_total == self.total;
        }

        if running_total > self.total {
            return false;
        }

        let next = self.values[pos];
        let next_pos = pos + 1;

        self.can_work_n(running_total + next, next_pos)
            || self.can_work_n(running_total * next, next_pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(3749, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
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
