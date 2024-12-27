use core::str;
use std::{
    cmp::Reverse, i64, iter::{self, Sum}, ops::Add, time::Instant
};

use aoclib::timing;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let SecretNumber(total) = txt
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(SecretNumber)
        .map(|sn| sn.into_iter().nth(2000 - 1).unwrap())
        .sum();
    total
}

fn part2(txt: &str) -> i64 {
    let mut items: HashMap<(i8, i8, i8, i8), i64> = HashMap::new();

    for sn in txt
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .map(SecretNumber)
    {
        let mut seen: HashSet<_> = HashSet::new();

        for ps in sn.price_sequence().take(2000) {
            if seen.insert(ps.deltas.clone()) {
                if let Some(existing) = items.get_mut(&ps.deltas) {
                    *existing += ps.price() as i64;
                } else {
                    items.insert(ps.deltas.clone(), ps.price() as i64);
                }
            }
        }
    }

    let (_, v) = items
        .iter()
        .sorted_by_key(|(_, &v)| Reverse(v))
        .take(1)
        .next()
        .unwrap();
    *v
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SecretNumber(i64);

impl SecretNumber {
    fn mix(&self, n: i64) -> SecretNumber {
        SecretNumber(self.0 ^ n)
    }

    fn prune(&self) -> SecretNumber {
        SecretNumber(self.0 % 16777216)
    }

    fn ones_digit(&self) -> i8 {
        let SecretNumber(n) = self;
        (*n % 10) as i8
    }

    fn prices(&self) -> impl Iterator<Item = i8> {
        itertools::chain(iter::repeat_n(self.clone(), 1), self.clone().into_iter())
            .map(|n| n.ones_digit())
    }

    fn price_sequence(&self) -> impl Iterator<Item = PriceSequence> {
        self.prices().tuple_windows().map(|seq| PriceSequence {
            prices: seq,
            deltas: (seq.1 - seq.0, seq.2 - seq.1, seq.3 - seq.2, seq.4 - seq.3),
        })
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct PriceSequence {
    prices: (i8, i8, i8, i8, i8),
    deltas: (i8, i8, i8, i8),
}

impl PriceSequence {
    fn price(&self) -> i8 {
        self.prices.4
    }
}

#[cfg(test)]
mod tests {
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
        let next_10: Vec<_> = s.into_iter().take(10).collect();

        assert_eq!(
            next_10,
            vec![
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
            ]
        )
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");

        assert_eq!(
            SecretNumber(8685429),
            SecretNumber(1).into_iter().nth(2000 - 1).unwrap()
        );
        assert_eq!(
            SecretNumber(4700978),
            SecretNumber(10).into_iter().nth(2000 - 1).unwrap()
        );
        assert_eq!(
            SecretNumber(15273692),
            SecretNumber(100).into_iter().nth(2000 - 1).unwrap()
        );
        assert_eq!(
            SecretNumber(8667524),
            SecretNumber(2024).into_iter().nth(2000 - 1).unwrap()
        );

        assert_eq!(37327623, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(20411980517, part1(test_input));
    }

    #[test]
    fn test_secret_ones() {
        assert_eq!(0, SecretNumber(10000).ones_digit());
        assert_eq!(1, SecretNumber(10001).ones_digit());
        assert_eq!(3, SecretNumber(3).ones_digit());
        assert_eq!(9, SecretNumber(4565469).ones_digit());
    }

    #[test]
    fn test_prices() {
        let sn = SecretNumber(123);
        let PriceSequence { prices, .. } = sn.price_sequence().next().unwrap();
        assert_eq!(prices, (3, 0, 6, 5, 4));
    }

    #[test]
    fn test_price_deltas() {
        let sn = SecretNumber(123);
        let mut itr = sn.price_sequence();
        let PriceSequence { deltas, .. } = itr.next().unwrap();
        assert_eq!((-3, 6, -1, -1), deltas);
    }

    #[test]
    fn test_input_pt2() {
        let test_input = "1\n2\n3\n2024";
        assert_eq!(23, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(2362, part2(test_input));
    }
}
