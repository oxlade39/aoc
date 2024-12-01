use std::{collections::HashSet, str::FromStr, time::Instant};

use aoclib::input;

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
    input::empty_line_chunks(txt)
        .map(|chunk| chunk.parse::<Answers>().unwrap())
        .map(|answers| answers.num_yes())
        .sum()
}

fn part2(txt: &str) -> usize {
    0
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Answers {
    yes_answers: HashSet<char>,
}

impl Answers {
    fn num_yes(&self) -> usize {
        self.yes_answers.len()
    }
}

impl FromStr for Answers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let yes_answers = s
            .lines()
            .flat_map(|line| line.chars())
            .collect::<HashSet<char>>();

        Ok(Answers { yes_answers })
    }
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
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
