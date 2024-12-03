use std::{sync::LazyLock, time::Instant};

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

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn part1(txt: &str) -> i64 {
    RE.captures_iter(&txt)
        .map(|c| c.extract())
        .fold(0, |accum, (_, [left, right])| {
            let l: i64 = left.parse().unwrap();
            let r: i64 = right.parse().unwrap();
            accum + l * r
        })
}

fn part2(txt: &str) -> i64 {
    txt.split("do()")
        .filter_map(|do_chunk| do_chunk.split("don't()").next())
        .map(part1)
        .sum()
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
