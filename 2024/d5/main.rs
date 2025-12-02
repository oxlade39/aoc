use core::str;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use aoclib::input;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!(
        "{:.2}ms",
        (now.elapsed().subsec_nanos() as f32) / 1_000_000 as f32
    );
}

fn part1(txt: &str) -> i64 {
    let parts = input::empty_line_chunks(txt).collect_vec();
    let ord: PageOrdering = parts[0].parse().unwrap();

    let mut count = 0;

    for list in parts[1].lines().map(|l| {
        l.split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect_vec()
    }) {
        let copy = ord.sorted(&list);

        if list == copy {
            let middle = list[list.len() / 2];
            count += middle;
        }
    }
    count
}

fn part2(txt: &str) -> i64 {
    let parts = input::empty_line_chunks(txt).collect_vec();
    let ord: PageOrdering = parts[0].parse().unwrap();

    let mut count = 0;

    for list in parts[1].lines().map(|l| {
        l.split(",")
            .map(|i| i.parse::<i64>().unwrap())
            .collect_vec()
    }) {
        let copy = ord.sorted(&list);

        if list != copy {
            let middle = copy[copy.len() / 2];
            count += middle;
        }
    }
    count
}

struct PageOrdering {
    /// values are before key
    befores: HashMap<i64, HashSet<i64>>,
    /// values are after key
    afters: HashMap<i64, HashSet<i64>>,
}

impl PageOrdering {
    fn sorted(&self, pages: &Vec<i64>) -> Vec<i64> {
        let mut copy = pages.clone();
        copy.sort_by(|l, r| {
            if let Some(mapping) = self.befores.get(r) {
                if mapping.contains(l) {
                    return Ordering::Less;
                }
            }
            if let Some(mapping) = self.afters.get(r) {
                if mapping.contains(l) {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        });
        copy
    }
}

impl FromStr for PageOrdering {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut befores: HashMap<i64, HashSet<i64>> = HashMap::new();
        let mut afters: HashMap<i64, HashSet<i64>> = HashMap::new();

        for order in s.lines().map(|l| {
            l.split("|")
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        }) {
            let left = order[0];
            let right = order[1];

            match befores.get_mut(&right) {
                Some(existing) => {
                    existing.insert(left);
                }
                _ => {
                    befores.insert(right, HashSet::from_iter([left]));
                }
            }

            match afters.get_mut(&left) {
                Some(existing) => {
                    existing.insert(right);
                }
                _ => {
                    afters.insert(left, HashSet::from_iter([right]));
                }
            }
        }
        Ok(Self { befores, afters })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(143, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(5108, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(123, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(7380, part2(test_input));
    }
}
