use core::str;
use std::{cmp::Ordering, collections::{HashMap, HashSet}, time::Instant};

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
    let mut befores: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut afters: HashMap<i64, HashSet<i64>> = HashMap::new();

    for order in parts[0]
        .lines()
        .map(|l| l.split("|").map(|n| n.parse::<i64>().unwrap()).collect_vec()) {

        let left = order[0];
        let right = order[1];
        
        if let Some(existing) = befores.get_mut(&right) {
            existing.insert(left);
        } else {
            befores.insert(right, HashSet::from_iter([left]));
        }

        if let Some(existing) = afters.get_mut(&left) {
            existing.insert(right);
        } else {
            afters.insert(left, HashSet::from_iter([right]));
        }
    }

    let mut count = 0;

    for list in parts[1]
        .lines()
        .map(|l| l.split(",").map(|i| i.parse::<i64>().unwrap()).collect_vec()) {

        let mut copy = list.clone();
        copy.sort_by(|l,r| {
            if let Some(mapping) = befores.get(r) {
                if mapping.contains(l) {
                    return Ordering::Less;
                }
            }
            if let Some(mapping) = afters.get(r) {
                if mapping.contains(l) {
                    return Ordering::Greater;
                }
            }

            // if let Some(mapping) = befores.get(l) {
            //     if mapping.contains(r) {
            //         return Ordering::Greater;
            //     }
            // }
            // if let Some(mapping) = afters.get(l) {
            //     if mapping.contains(r) {
            //         return Ordering::Less;
            //     }
            // }
            println!("oops: {},{}", l, r);
            return Ordering::Equal;
        });

        if list == copy {
            let middle = list[list.len() / 2];
            // println!("ordered:\n{:?}\nmiddle: {:?}", list, middle);
            count += middle;
        }
    }


    count
}

fn part2(txt: &str) -> i64 {
    let parts = input::empty_line_chunks(txt).collect_vec();
    let mut befores: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut afters: HashMap<i64, HashSet<i64>> = HashMap::new();

    for order in parts[0]
        .lines()
        .map(|l| l.split("|").map(|n| n.parse::<i64>().unwrap()).collect_vec()) {

        let left = order[0];
        let right = order[1];
        
        if let Some(existing) = befores.get_mut(&right) {
            existing.insert(left);
        } else {
            befores.insert(right, HashSet::from_iter([left]));
        }

        if let Some(existing) = afters.get_mut(&left) {
            existing.insert(right);
        } else {
            afters.insert(left, HashSet::from_iter([right]));
        }
    }

    let mut count = 0;

    for list in parts[1]
        .lines()
        .map(|l| l.split(",").map(|i| i.parse::<i64>().unwrap()).collect_vec()) {

        let mut copy = list.clone();
        copy.sort_by(|l,r| {
            if let Some(mapping) = befores.get(r) {
                if mapping.contains(l) {
                    return Ordering::Less;
                }
            }
            if let Some(mapping) = afters.get(r) {
                if mapping.contains(l) {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        });

        if list != copy {
            let middle = copy[copy.len() / 2];
            count += middle;
        }
    }
    count
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
        assert_eq!(0, part2(test_input));
    }
}
