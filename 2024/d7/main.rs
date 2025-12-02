use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!(
        "{:.2}ms",
        (now.elapsed().subsec_nanos() as f32) / 1_000_000 as f32
    );
}

fn part1(txt: &str) -> i64 {
    solve(txt, [Operation::Plus, Operation::Mul])
}

fn part2(txt: &str) -> i64 {
    solve(txt, [Operation::Plus, Operation::Mul, Operation::Concat])
}

fn solve<const N: usize>(txt: &str, operations: [Operation; N]) -> i64 {
    txt.lines()
        .map(|l| l.parse::<Calibration>().unwrap())
        .filter(|c| c.can_work(0, 0, operations))
        .map(|c| c.total)
        .sum()
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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Mul,
    Concat,
}

impl Operation {
    fn exec(&self, x: (i64, i64)) -> i64 {
        match self {
            Operation::Plus => x.0 + x.1,
            Operation::Mul => x.0 * x.1,
            Operation::Concat => {
                // slow version:
                // format!("{}{}", x.0, x.1).parse().unwrap()
                let right_digits = (x.1 as f64).log10().floor() as u32 + 1;
                x.0 * 10_i64.pow(right_digits) + x.1
            }
        }
    }
}

impl Calibration {
    fn can_work<const N: usize>(
        &self,
        running_total: i64,
        position: usize,
        operands: [Operation; N],
    ) -> bool {
        if position == self.values.len() {
            return running_total == self.total;
        }

        if running_total > self.total {
            return false;
        }

        let next = self.values[position];
        let next_pos = position + 1;

        operands
            .iter()
            .any(|op| self.can_work(op.exec((running_total, next)), next_pos, operands))
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
        assert_eq!(1038838357795, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(11387, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(254136560217241, part2(test_input));
    }
}
