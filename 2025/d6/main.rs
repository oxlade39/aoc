use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{grid::{Grid, GridPosition}, input, timing};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let grid: Vec<Vec<Col>> = txt.lines()
        .map(|l| l.split_whitespace().map(|c| c.parse::<Col>().expect("col")).collect())
        .collect();
    let g = Grid { rows: grid };

    let operations: Vec<_> = g.right_from(GridPosition { col: 0, row: g.height() - 1 }).collect();
    let mut sum = 0;
    for i in 0..g.width() {
        let p = GridPosition::new(i, 0);
        let (_, operation) = operations[i];
        let init = match operation {
            Col::Number(_) => panic!("number in last row"),
            Col::Mul => 1,
            Col::Add => 0,
        };
        sum += g.down_from(p).fold(init, |accum, (_, item)| {
            match item {
                Col::Number(n) => match operation {
                    Col::Number(_) => panic!("impossible"),
                    Col::Mul => n * accum,
                    Col::Add => n + accum,
                },
                _ => accum,
            }
        });
    }

    sum
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Col {
    Number(i64),
    Mul,
    Add
}

impl FromStr for Col {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Col::Mul),
            "+" => Ok(Col::Add),
            other => Ok(Col::Number(other.parse().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(4277556, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(6635273135233, part1(test_input));
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
