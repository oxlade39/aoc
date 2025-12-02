use std::time::Instant;

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

fn part1(txt: &str) -> i32 {
    let target = 2020;
    let expenses = txt
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().expect(&format!("integer but was '{l}'")))
        .collect_vec();

    if let Some(i) = find_pair(target, &expenses) {
        i
    } else {
        0
    }
}

fn part2(txt: &str) -> i32 {
    let target = 2020;
    let expenses = txt
        .trim()
        .lines()
        .map(|l| l.parse::<i32>().expect(&format!("integer but was '{l}'")))
        .filter(|i| *i < target)
        .collect_vec();

    for i in expenses.iter() {
        let remainder = target - *i;
        if let Some(j) = find_pair(remainder, &expenses) {
            return i * j;
        }
    }

    0
}

fn find_pair(target: i32, within: &Vec<i32>) -> Option<i32> {
    let filtered = within.into_iter().filter(|i| **i < target).collect_vec();

    for i in filtered.iter() {
        let remainder = target - *i;
        if filtered.iter().any(|j| **j == remainder) {
            return Some(**i * remainder);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(514579, part1(test_input));
    }

    #[test]
    fn sample_input_pt2() {
        assert_eq!(241861950, part2(include_str!("input.test.txt")));
    }
}
