use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

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
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [left, right]) in re.captures_iter(&txt).map(|c| c.extract()) {
        let l: i64 = left.parse().unwrap();
        let r: i64 = right.parse().unwrap();
        if l <= 999 && r <= 999 {
            sum += l * r
        }        
    }
    sum
}

fn part2(txt: &str) -> i64 {
    let mut sum = 0;
    let do_chunks = txt.split("do()").collect_vec();

    for chunk in do_chunks {
        let sub = chunk.split("don't()").collect_vec();
        sum += part1(sub[0]);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(161, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(189600467, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test2.txt");
        assert_eq!(48, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(107069718, part2(test_input));
    }
}
