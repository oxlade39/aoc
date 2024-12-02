use std::{str::FromStr, time::Instant};

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
    txt.lines()
    .map(|l| l.parse::<Report>().unwrap())
    .map(|report| Part1(report))
    .filter(|pt| pt.is_safe())
    .count()
}

fn part2(txt: &str) -> usize {
    txt.lines()
        .map(|l| l.parse::<Report>().unwrap())
        .map(|report| Part2(report))
        .filter(|pt| pt.is_safe())
        .count()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Report {
    levels: Vec<usize>,
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Ok(Report { levels })
    }
}

trait SafetyReport {
    fn is_safe(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part1(Report);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part2(Report);

impl SafetyReport for Part1 {
    fn is_safe(&self) -> bool {
        all_safe(&self.0.levels)
    }
}

impl SafetyReport for Part2 {
    fn is_safe(&self) -> bool {
        any_safe(&self.0)
    }
}

fn all_safe(r: &Vec<usize>) -> bool {
    let all_increasing = r.iter().copied()
        .tuple_windows()
        .all(|(left, right)| (right as i64 - left as i64).signum() == 1);
    let all_decreasing = r.iter().copied()
        .tuple_windows()
        .all(|(left, right)| (right as i64 - left as i64).signum() == -1);
    let all_in_range = r.iter().copied()
        .tuple_windows()
        .all(|(left, right)| (left as i64).abs_diff(right as i64) > 0 && (left as i64).abs_diff(right as i64) <= 3);

    (all_increasing || all_decreasing) && all_in_range
}

fn any_safe(r: &Report) -> bool {
    (0..r.levels.len()).any(|i| {
        let mut without = r.levels.clone();
        without.remove(i);
        all_safe(&without)
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_input_pt1_lines() {
        assert_eq!(1, part1("7 6 4 2 1"));
    }

    #[test]
    fn sample_input2_pt2() {
        // 7 6 4 2 1: Safe without removing any level.
        let report = "7 6 4 2 1".parse::<Report>().unwrap();
        assert_eq!(true, Part2(report).is_safe());

        // 1 2 7 8 9: Unsafe regardless of which level is removed.
        let report = "1 2 7 8 9".parse::<Report>().unwrap();
        assert_eq!(false, Part2(report).is_safe());

        // 9 7 6 2 1: Unsafe regardless of which level is removed.
        let report = "9 7 6 2 1".parse::<Report>().unwrap();
        assert_eq!(false, Part2(report).is_safe());

        // 1 3 2 4 5: Safe by removing the second level, 3.
        let report = "1 3 2 4 5".parse::<Report>().unwrap();
        assert_eq!(true, Part2(report).is_safe());

        // 8 6 4 4 1: Safe by removing the third level, 4.
        let report = "8 6 4 4 1".parse::<Report>().unwrap();
        assert_eq!(true, Part2(report).is_safe());

        // 1 3 6 7 9: Safe without removing any level.
        let report = "1 3 6 7 9".parse::<Report>().unwrap();
        assert_eq!(true, Part2(report).is_safe());
    }

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(246, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(4, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(318, part2(test_input));
    }
}
