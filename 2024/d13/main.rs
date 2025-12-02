use core::str;
use std::{i64, str::FromStr, time::Instant};

use aoclib::{input, timing};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    input::empty_line_chunks(txt)
        .map(|chunk| chunk.parse::<ClawMachine>().unwrap())
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|(x, y)| 3 * x + y)
        .sum()
}

fn part2(txt: &str) -> i64 {
    input::empty_line_chunks(txt)
        .map(|chunk| chunk.parse::<ClawMachine>().unwrap())
        .map(|claw| {
            ClawMachine(
                claw.0,
                claw.1,
                Prize {
                    x: claw.2.x + 10000000000000,
                    y: claw.2.y + 10000000000000,
                },
            )
        })
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|(x, y)| 3 * x + y)
        .sum()
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Button {
    name: char,
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct ClawMachine(Button, Button, Prize);

impl ClawMachine {
    fn solve(&self) -> Option<(i64, i64)> {
        let a = self.0;
        let b = self.1;
        let prize = self.2;

        // rearrange simultaneous equation

        let determinant = a.x * b.y - a.y * b.x;

        if determinant == 0 {
            return None;
        }

        let x = (prize.x * b.y - prize.y * b.x) / determinant;
        let y = (a.x * prize.y - a.y * prize.x) / determinant;

        if (a.x * x) + (b.x * y) == prize.x && (a.y * x) + (b.y * y) == prize.y {
            Some((x, y))
        } else {
            None
        }
    }
}

impl FromStr for Button {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(": ").unwrap();
        let name = left.split_once(" ").unwrap().1.chars().next().unwrap();

        let (x_part, y_part) = right.split_once(", ").unwrap();
        let (_, r) = x_part.split_once("+").unwrap();
        let x = r.parse().expect(&format!("{:?}", r));
        let y = y_part.split_once("+").unwrap().1.parse().unwrap();

        Ok(Button { name, x, y })
    }
}

impl FromStr for Prize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, right) = s.split_once(": ").unwrap();
        let (x_part, y_part) = right.split_once(", ").unwrap();
        let x = x_part.split_once("=").unwrap().1.parse().unwrap();
        let y = y_part.split_once("=").unwrap().1.parse().unwrap();
        Ok(Prize { x, y })
    }
}

impl FromStr for ClawMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().take(3).collect_vec();
        Ok(ClawMachine(
            lines[0].parse().expect("button a"),
            lines[1].parse().expect("button b"),
            lines[2].parse().expect("priza"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_multiples() {
        let c = ClawMachine(
            Button {
                name: 'A',
                x: 94,
                y: 34,
            },
            Button {
                name: 'B',
                x: 22,
                y: 67,
            },
            Prize { x: 8400, y: 5400 },
        );
        let result = c.solve();
        println!("result: {:?}", result);
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(480, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(31761, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(90798500745591, part2(test_input));
    }
}
