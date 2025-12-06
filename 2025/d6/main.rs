use core::str;
use std::{i64, str::FromStr, time::Instant};

use aoclib::{
    grid::{Grid, GridPosition},
    timing,
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let grid: Vec<Vec<Col>> = txt
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<Col>().expect("col"))
                .collect()
        })
        .collect();
    let g = Grid { rows: grid };

    let operations: Vec<_> = g
        .right_from(GridPosition {
            col: 0,
            row: g.height() - 1,
        })
        .collect();
    let mut sum = 0;
    for i in 0..g.width() {
        let p = GridPosition::new(i, 0);
        let (_, operation) = operations[i];
        let init = match operation {
            Col::Number(_) => panic!("number in last row"),
            Col::Mul => 1,
            Col::Add => 0,
        };
        sum += g.down_from(p).fold(init, |accum, (_, item)| match item {
            Col::Number(n) => match operation {
                Col::Number(_) => panic!("impossible"),
                Col::Mul => n * accum,
                Col::Add => n + accum,
            },
            _ => accum,
        });
    }

    sum
}

fn part2(txt: &str) -> i64 {
    let grid: Vec<Vec<Col2>> = txt
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    ' ' => Col2::Space,
                    '*' => Col2::Mul,
                    '+' => Col2::Add,
                    other => Col2::Number(other.to_digit(10).unwrap() as i64),
                })
                .collect()
        })
        .collect();
    let g = Grid { rows: grid };

    let bottom_right_number = GridPosition::new(g.width() - 1, g.height() - 1);
    let mut total: i64 = 0;

    let mut skip_space = false;
    let mut numbers = Vec::new();
    for (p, col) in g.left_from(bottom_right_number) {
        match col {
            Col2::Space => {
                if skip_space {
                    skip_space = false;
                } else {
                    numbers.push(to_n(&g, p));
                }
            }
            Col2::Mul => {
                numbers.push(to_n(&g, p));
                skip_space = true;
                let prod: i64 = numbers.iter().product();
                // println!("{:?} = {}", numbers, prod);
                total += prod;
                numbers.clear();
            }
            Col2::Add => {
                numbers.push(to_n(&g, p));
                skip_space = true;
                let sum: i64 = numbers.iter().sum();
                // println!("{:?} = {}", numbers, sum);
                total += sum;
                numbers.clear();
            }
            Col2::Number(_) => panic!("no numbers bottom row"),
        }
    }

    total
}

fn to_n(g: &Grid<Col2>, p: GridPosition) -> i64 {
    let mut num = 0;
    for (units, value) in g
        .up_from(p)
        .skip(1)
        .filter_map(|(_, up_col)| match up_col {
            Col2::Number(n) => Some(*n),
            _ => None,
        })
        .enumerate()
    {
        let n = 10_i64.pow(units as u32) * value;
        // println!("up col {:?} value {}, n {}", up_col, value, n);
        num += n;
    }
    // println!("adding {}", num);
    num
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Col {
    Number(i64),
    Mul,
    Add,
}

impl FromStr for Col {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Col::Mul),
            "+" => Ok(Col::Add),
            other => Ok(Col::Number(other.parse().unwrap())),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Col2 {
    Number(i64),
    Mul,
    Add,
    Space,
}

impl FromStr for Col2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Col2::Mul),
            "+" => Ok(Col2::Add),
            " " => Ok(Col2::Space),
            other => Ok(Col2::Number(other.parse().unwrap())),
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
        assert_eq!(3263827, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(12542543681221, part2(test_input));
    }
}
