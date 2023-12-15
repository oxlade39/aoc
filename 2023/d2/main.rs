use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(txt: &str) -> i32 {
    let bag = {
        let mut cubes = HashMap::new();
        cubes.insert("red".to_owned(), 12);
        cubes.insert("green".to_owned(), 13);
        cubes.insert("blue".to_owned(), 14);
        Bag { cubes }
    };

    txt.lines()
        .map(|l| l.parse::<Game>().expect(l))
        .enumerate()
        .filter(|(_i, g)| g.draws.iter().all(|d| bag.possible(d)))
        .map(|(i, _)| (i + 1) as i32)
        .sum()
}

fn part2(txt: &str) -> i32 {
    txt.lines()
        .map(|l| l.parse::<Game>().expect(l))
        .map(|g| g.power())
        .sum()
}

#[derive(Debug, Clone)]
struct Game {
    draws: Vec<Draw>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Draw {
    cubes: HashMap<String, i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bag {
    cubes: HashMap<String, i32>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outter = s.split(": ").nth(1).expect("': '");
        let inner = outter
            .split("; ")
            .map(|draw| {
                let cube_counts: HashMap<String, i32> = draw
                    .split(", ")
                    .map(|cc| {
                        let parts = cc.split(" ").collect_vec();
                        (parts[1].to_owned(), parts[0].parse::<i32>().expect("count"))
                    })
                    .collect();
                Draw { cubes: cube_counts }
            })
            .collect_vec();

        Ok(Game { draws: inner })
    }
}

impl Game {
    fn power(&self) -> i32 {
        let mut maxs: HashMap<String, i32> = HashMap::new();
        for d in &self.draws {
            for (k, v) in &d.cubes {
                if let Some(existing) = maxs.remove(k) {
                    maxs.insert(k.clone(), (*v).max(existing));
                } else {
                    maxs.insert(k.clone(), *v);
                }
            }
        }
        maxs.values().product()
    }
}

impl Bag {
    fn possible(&self, draw: &Draw) -> bool {
        for (k, v) in &draw.cubes {
            if let Some(count) = self.cubes.get(k) {
                if *v > *count {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(8, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(2286, part2(include_str!("input.test.txt")));
    }
}
