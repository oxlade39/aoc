use core::str;
use std::{i64, iter::{self, Sum}, ops::{Add, Mul}, str::FromStr, time::Instant, usize};

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
    let SecretNumber(total) = txt.lines().map(|l| l.parse::<i64>().unwrap()).map(SecretNumber)
        .map(|sn| sn.into_iter().nth(2000 - 1).unwrap())
        .sum();
    total
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct SecretNumber(i64);

impl SecretNumber {

    fn mix(&self, n: i64) -> SecretNumber {
        // To mix a value into the secret number, 
        // calculate the bitwise XOR of the given value and the secret number. 
        // Then, the secret number becomes the result of that operation. 
        // (If the secret number is 42 and you were to mix 15 into the secret number, 
        // the secret number would become 37.)
        SecretNumber(self.0 ^ n)
    }

    fn prune(&self) -> SecretNumber {
        // To prune the secret number, 
        // calculate the value of the secret number modulo 16777216. 
        // Then, the secret number becomes the result of that operation. 
        // (If the secret number is 100000000 and you were to prune the secret number, 
        // the secret number would become 16113920.)
        SecretNumber(self.0 % 16777216)
    }
}

impl Add for SecretNumber {
    type Output = SecretNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sum for SecretNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SecretNumber(0), |acc, sn| acc + sn)
    }
}

struct SecretNumberIter(SecretNumber);

impl Iterator for SecretNumber {
    type Item = SecretNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let SecretNumber(n) = self;
        let result = *n * 64;
        let step_one = self.mix(result).prune();
        let result = step_one.0 / 32;
        let step_two = step_one.mix(result).prune();
        let result = step_two.0 * 2048;
        let step_three = step_two.mix(result).prune();

        self.0 = step_three.0;

        Some(step_three)
    }
}

#[cfg(test)]
mod tests {    
    use std::iter;

    use crate::*;

    #[test]
    fn test_mix() {
        assert_eq!(SecretNumber(37), SecretNumber(42).mix(15));
    }

    #[test]
    fn test_prune() {
        assert_eq!(SecretNumber(16113920), SecretNumber(100000000).prune());
    }

    #[test]
    fn test_123() {
        let s = SecretNumber(123);
        let next_10: Vec<_> = s.into_iter()
            .take(10)
            .collect();

        assert_eq!(next_10, vec![
            SecretNumber(15887950),
            SecretNumber(16495136),
            SecretNumber(527345),
            SecretNumber(704524),
            SecretNumber(1553684),
            SecretNumber(12683156),
            SecretNumber(11100544),
            SecretNumber(12249484),
            SecretNumber(7753432),
            SecretNumber(5908254),
        ])
        
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");

        assert_eq!(SecretNumber(8685429), SecretNumber(1).into_iter().nth(2000 - 1).unwrap());
        assert_eq!(SecretNumber(4700978), SecretNumber(10).into_iter().nth(2000 - 1).unwrap());
        assert_eq!(SecretNumber(15273692), SecretNumber(100).into_iter().nth(2000 - 1).unwrap());
        assert_eq!(SecretNumber(8667524), SecretNumber(2024).into_iter().nth(2000 - 1).unwrap());

        assert_eq!(37327623, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(20411980517, part1(test_input));
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
