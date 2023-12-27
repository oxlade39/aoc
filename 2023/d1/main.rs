use std::{collections::BTreeMap, time::Instant};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i32 {
    const RADIX: u32 = 10;

    txt.lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.to_digit(RADIX).is_some())
                .collect_vec()
        })
        .map(|l| {
            // if (l.len() < 2) {
            // println!("bad: {:?}", l);
            // }
            format!("{}{}", l.first().unwrap(), l.last().unwrap())
        })
        .map(|i| i.parse::<i32>().unwrap())
        .sum()
}

fn part2(txt: &str) -> i32 {
    let n = [
        ["one", "1"],
        ["two", "2"],
        ["three", "3"],
        ["four", "4"],
        ["five", "5"],
        ["six", "6"],
        ["seven", "7"],
        ["eight", "8"],
        ["nine", "9"],
    ];

    let mut sum = 0;

    for line in txt.lines() {
        let mut forward = BTreeMap::new();
        let mut back = BTreeMap::new();

        for (n, [word, num]) in n.iter().enumerate() {
            if let Some(i) = line.find(word) {
                forward.insert(i, n + 1);
            }
            if let Some(i) = line.find(num) {
                forward.insert(i, n + 1);
            }
            if let Some(i) = line.rfind(word) {
                back.insert(i, n + 1);
            }
            if let Some(i) = line.rfind(num) {
                back.insert(i, n + 1);
            }
        }

        sum += forward.pop_first().expect(line).1 * 10;
        sum += back.pop_last().expect(line).1;
    }
    sum as i32
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_input_pt1() {
        let input = include_str!("input.test.txt");
        assert_eq!(142, part1(input));
    }

    #[test]
    fn sample_input_pt2() {
        let input = include_str!("input2.test.txt");
        assert_eq!(281, part2(input));
    }
}
