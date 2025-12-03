use core::str;
use std::{str::FromStr, time::Instant};

use aoclib::timing;
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
        .map(|b| b.joltage_n(2))
        .sum()
}

fn part2(txt: &str) -> usize {
    txt.lines()
        .map(|l| l.parse::<Bank>().expect("Bank"))
        .map(|b| b.joltage_n(12))
        .sum()
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
            Err(_) => Err(format!("bad input '{}'", s)),
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
    fn joltage_n(&self, n: usize) -> usize {
        let positions: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .sorted_by(|(a_pos, a), (b_pos, b)| a.cmp(b).then(b_pos.cmp(a_pos)))
            .collect();

        let mut j = 0;
        let mut last_filled_pos_from_left = None;

        for i in 0..n {
            let mut positions = positions.clone();
            let required_pos = n - 1 - i;
            while let Some((position_from_left, next)) = positions.pop() {
                let pos_from_right = self.0.len() - position_from_left;
                if pos_from_right > required_pos {
                    if last_filled_pos_from_left.is_none() {
                        let num = 10_usize.pow(required_pos as u32) * next.0;
                        j += num;
                        last_filled_pos_from_left = Some(
                            last_filled_pos_from_left
                                .unwrap_or(0)
                                .max(position_from_left),
                        );
                        break;
                    } else if position_from_left > last_filled_pos_from_left.unwrap_or(0) {
                        let num = 10_usize.pow(required_pos as u32) * next.0;
                        j += num;
                        last_filled_pos_from_left = Some(
                            last_filled_pos_from_left
                                .unwrap_or(0)
                                .max(position_from_left),
                        );
                        break;
                    }
                }
            }
        }
        j
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_bank_parse() {
        let s = "987654321111111";
        let b = s.parse::<Bank>();
        assert_eq!(
            Ok(Bank(vec![
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
            ])),
            b
        );
    }

    #[test]
    fn test_bank_joltage_n() {
        let s = "987654321111111";
        let b = s.parse::<Bank>().unwrap();
        assert_eq!(98, b.joltage_n(2));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(357, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(16927, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(3121910778619, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(167384358365132, part2(test_input));
    }
}
