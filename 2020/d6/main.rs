use std::{collections::HashSet, fmt::Debug, str::FromStr, time::Instant};

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
    count_yes::<AnyYes>(txt)
}

fn part2(txt: &str) -> usize {
    count_yes::<AllYes>(txt)
}

fn count_yes<T>(txt: &str) -> usize
where
    T: CountYes,
    T: FromStr,
    T::Err: Debug,
{
    input::empty_line_chunks(txt)
        .map(|chunk| chunk.parse::<T>().expect("urgh"))
        .map(|answers| answers.num_yes())
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnyYes {
    yes_answers: HashSet<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AllYes {
    yes_answers: HashSet<char>,
}

trait CountYes {
    fn num_yes(&self) -> usize;
}

impl CountYes for AnyYes {
    fn num_yes(&self) -> usize {
        self.yes_answers.len()
    }
}

impl CountYes for AllYes {
    fn num_yes(&self) -> usize {
        self.yes_answers.len()
    }
}

impl FromStr for AnyYes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let yes_answers = s
            .lines()
            .flat_map(|line| line.chars())
            .collect::<HashSet<char>>();

        Ok(AnyYes { yes_answers })
    }
}

impl FromStr for AllYes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let yes_answers = s
            .lines()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|accum, e| accum.intersection(&e).copied().collect())
            .unwrap();

        Ok(AllYes { yes_answers })
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
        assert_eq!(6748, part1(test_input));
    }

    #[test]
    fn same_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(6, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(3445, part2(test_input));
    }
}
