use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(txt: &str) -> i64 {
    // let cards = txt.lines()
    //     .map(|l| l.parse::<Card>().unwrap())
    //     .collect_vec();

    // for (i, c) in cards.iter().enumerate() {
    //     println!("card: {:?}: {:?} points", i + 1, c.points());
    //     println!("");
    // }
    

    txt.lines()
        .map(|l| l.parse::<Card>().unwrap())
        .map(|card| card.points())
        .sum()
}

fn part2(txt: &str) -> i32 {
    -1
}

#[derive(Debug)]
struct Card {
    numbers: HashSet<i64>,
    winners: HashSet<i64>
}

impl Card {
    fn points(&self) -> i64 {
        let intersection = self.numbers.intersection(&self.winners);
        let count = intersection.count();
        if count == 0 {
            0
        } else {
            2_i64.pow((count - 1) as u32)
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" | ").collect_vec();
        
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
        assert_eq!(-1, part2(include_str!("input.test.txt")));
    }
}
