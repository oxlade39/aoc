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
        .map(|l| l.parse::<PasswordPolicy>().expect("password policy"))
        .filter(|p| p.passes())
        .count()
}

fn part2(txt: &str) -> i32 {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PasswordPolicy {
    upper: i32,
    lower: i32,
    c: char,
    password: String,
}

impl PasswordPolicy {
    fn passes(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.c {
                count += 1;
            }
        }

        count >= self.lower && count <= self.upper
    }
}

impl FromStr for PasswordPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outer_parts = s.split(" ").collect_vec();
        if outer_parts.len() != 3 {
            return Err(format!("expected 3 parts but was {}", outer_parts.len()));
        }

        let (lower, upper) = outer_parts[0]
            .split("-")
            .map(|i| i.parse::<i32>().expect("upper and lower"))
            .collect_tuple()
            .unwrap();

        let c = outer_parts[1].chars().next().expect("a password char");
        let password = outer_parts[2].to_owned();

        Ok(PasswordPolicy {
            upper,
            lower,
            c,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse() {
        let test_input = include_str!("input.test.txt")
            .lines()
            .map(|l| l.parse::<PasswordPolicy>().unwrap())
            .collect_vec();

        let expected = vec![
            PasswordPolicy {
                lower: 1,
                upper: 3,
                c: 'a',
                password: "abcde".to_owned(),
            },
            PasswordPolicy {
                lower: 1,
                upper: 3,
                c: 'b',
                password: "cdefg".to_owned(),
            },
            PasswordPolicy {
                lower: 2,
                upper: 9,
                c: 'c',
                password: "ccccccccc".to_owned(),
            },
        ];

        assert_eq!(expected, test_input);
    }

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2, part1(test_input));
    }

    #[test]
    fn input_pt1_answer() {
        let test_input = include_str!("input.txt");
        assert_eq!(416, part1(test_input));
    }

    #[test]
    fn sample_input_pt2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }
}
