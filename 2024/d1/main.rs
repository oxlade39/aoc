use std::{
    iter::zip,
    time::Instant,
};

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

fn part1(txt: &str) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for l in txt.lines() {
        let mut split = l.split("   ");
        left.push(split.next().unwrap().parse::<usize>().unwrap());
        right.push(split.next().unwrap().parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| l.max(r) - l.min(r)).sum()
}

fn part2(txt: &str) -> usize {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for l in txt.lines() {
        let mut split = l.split("   ");
        left.push(split.next().unwrap().parse::<usize>().unwrap());
        right.push(split.next().unwrap().parse::<usize>().unwrap());
    }

    let right_counts = right.iter().counts();
    let mut sum = 0;
    for i in left {
        let right_count = *right_counts.get(&i).unwrap_or(&0);
        sum += i * right_count;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(11, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1830467, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(31, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(26674158, part2(test_input));
    }
}
