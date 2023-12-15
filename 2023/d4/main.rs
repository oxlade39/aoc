use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.parse::<Card>().unwrap())
        .map(|card| card.points())
        .sum()
}

fn part2(txt: &str) -> u32 {
    let cards = txt
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect_vec();
    let mut card_counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        let card = &cards[i];
        let points = card.matches();
        for _ in 0..card_counts[i] {
            for j in (i + 1)..(i + 1 + points as usize) {
                card_counts[j] += 1;
            }
        }
    }
    card_counts.iter().sum()
}

#[derive(Debug)]
struct Card {
    numbers: HashSet<i64>,
    winners: HashSet<i64>,
}

impl Card {
    fn matches(&self) -> u32 {
        let intersection = self.numbers.intersection(&self.winners);
        intersection.count() as u32
    }

    fn points(&self) -> i64 {
        let count = *&self.matches();
        if count == 0 {
            0
        } else {
            2_i64.pow(count - 1)
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" | ").collect_vec();

        let winners = parts[0]
            .split(": ")
            .nth(1)
            .expect(": ")
            .split(" ")
            .filter_map(|n| n.parse::<i64>().ok())
            .collect();

        let my_numbers: HashSet<i64> = parts[1]
            .split(" ")
            .filter_map(|n| n.parse::<i64>().ok())
            .collect();

        Ok(Card {
            numbers: my_numbers,
            winners,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_points() {
        let p = Card {
            numbers: [1, 2].into_iter().collect(),
            winners: [3, 4].into_iter().collect(),
        };
        assert_eq!(0, p.points());

        let p = Card {
            numbers: [1, 2].into_iter().collect(),
            winners: [1, 4].into_iter().collect(),
        };
        assert_eq!(1, p.points());

        let p = Card {
            numbers: [1, 2].into_iter().collect(),
            winners: [1, 2].into_iter().collect(),
        };
        assert_eq!(2, p.points());

        let p = Card {
            numbers: [1, 2, 3, 4].into_iter().collect(),
            winners: [1, 2, 3, 4].into_iter().collect(),
        };
        assert_eq!(8, p.points());
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(13, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(30, part2(include_str!("input.test.txt")));
    }
}
