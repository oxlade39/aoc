use core::str;
use std::{str::FromStr, time::Instant};

use aoclib::timing;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let counter: Counter = txt
        .lines()
        .map(|l| l.parse::<Turn>().unwrap())
        .fold(Counter(Dial(50), 0), |accum, item| accum.step(item));

    counter.1
}

fn part2(txt: &str) -> i64 {
    let counter: Counter2 = txt
        .lines()
        .map(|l| l.parse::<Turn>().unwrap())
        .fold(Counter2(Dial(50), 0), |accum, item| accum.step(item));

    counter.1
}

#[derive(Debug, PartialEq)]
struct Dial(u64);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Left(u64),
    Right(u64),
}

impl FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let indicator = &s[0..1];
        let amount = &s[1..];
        match indicator {
            "L" => Ok(Turn::Left(amount.parse().unwrap())),
            "R" => Ok(Turn::Right(amount.parse().unwrap())),
            other => Err(format!("bad indicator [{}]", other)),
        }
    }
}

impl Dial {
    fn left(&self, amount: u64) -> Dial {
        let current = self.0 as i64;
        let amount = amount as i64;
        let moved = current - amount;
        let next_pos = moved.rem_euclid(100);
        Dial(next_pos as u64)
    }

    fn right(&self, amount: u64) -> Dial {
        let next_pos = (self.0 + amount) % 100;
        Dial(next_pos)
    }
}

#[derive(Debug, PartialEq)]
struct Counter(Dial, i64);

impl Counter {
    fn step(&self, turn: Turn) -> Counter {
        let dial = &self.0;
        let count = self.1;
        let after = match turn {
            Turn::Left(n) => dial.left(n),
            Turn::Right(n) => dial.right(n),
        };
        let increment = if after == Dial(0) { count + 1 } else { count };
        Counter(after, increment)
    }
}

#[derive(Debug, PartialEq)]
struct Counter2(Dial, i64);

impl Counter2 {
    fn step(&self, turn: Turn) -> Counter2 {
        let dial = &self.0;
        let count = self.1;

        let next = match turn {
            Turn::Left(n) => (dial.0 as i64) - n as i64,
            Turn::Right(n) => (dial.0 as i64) + n as i64,
        };
        let passes = next / 100;

        let prev_signum = (dial.0 as i64).signum();
        let next_signum = next.signum();

        let sign_change = match (prev_signum, next_signum) {
            (1, 0) => 1,
            (1, -1) => 1,
            (_, _) => 0,
        };

        // println!("next: {}, passes: {}, sign change {}", next, passes, sign_change);
        Counter2(
            Dial(next.rem_euclid(100) as u64),
            count + passes.abs() + sign_change,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_dial_turn() {
        let d = Dial(11);
        let d = d.right(8);
        assert_eq!(Dial(19), d);
        assert_eq!(Dial(0), d.left(19));
        assert_eq!(Dial(99), Dial(0).left(1));
        assert_eq!(Dial(0), Dial(52).right(48));
    }

    #[test]
    fn test_big_turn() {
        let c = Counter2(Dial(50), 0);
        let c1 = c.step(Turn::Right(1000));
        assert_eq!(10, c1.1);
    }

    #[test]
    fn test_land_zero() {
        let c = Counter2(Dial(0), 0);
        let c1 = c.step(Turn::Left(1));
        assert_eq!(0, c1.1);
    }

    #[test]
    fn test_input_pt1() {
        let mut counter = Counter(Dial(50), 0);
        let test_input = include_str!("input.test.txt");
        for turn in test_input.lines().map(|l| l.parse::<Turn>().unwrap()) {
            // dbg!("{:?} turning {:?}", &counter, &turn);
            counter = counter.step(turn);
            // dbg!("{:?}", &counter);
        }
        assert_eq!(3, counter.1);
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1078, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let mut counter = Counter2(Dial(50), 0);
        let test_input = include_str!("input.test.txt");
        for turn in test_input.lines().map(|l| l.parse::<Turn>().unwrap()) {
            // println!("{:?} turning {:?}", &counter, &turn);
            counter = counter.step(turn);
            // println!("{:?}", &counter);
        }
        assert_eq!(6, counter.1);
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(6412, part2(test_input));
    }
}
