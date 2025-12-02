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
    count(txt, Part1)
}

fn part2(txt: &str) -> usize {
    count(txt, Part2)
}

fn count<T: PasswordValidator>(txt: &str, validator: T) -> usize {
    txt.lines()
        .map(|l| l.parse::<PasswordPolicy>().expect("password policy"))
        .filter(|p| (&validator).valid(p))
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PasswordPolicy {
    upper: i32,
    lower: i32,
    c: char,
    password: String,
}

trait PasswordValidator {
    fn valid(&self, policy: &PasswordPolicy) -> bool;
}

struct Part1;
struct Part2;

impl PasswordValidator for Part1 {
    fn valid(&self, policy: &PasswordPolicy) -> bool {
        let mut count = 0;
        for c in policy.password.chars() {
            if c == policy.c {
                count += 1;
            }
        }

        count >= policy.lower && count <= policy.upper
    }
}

impl PasswordValidator for Part2 {
    fn valid(&self, policy: &PasswordPolicy) -> bool {
        let mut password_chars = policy.password.chars();
        let lower_offset = (policy.lower - 1) as usize;
        let upper_offset = ((policy.upper - policy.lower) - 1) as usize;
        let lower_char = password_chars.nth(lower_offset).filter(|c| *c == policy.c);
        let upper_char = password_chars.nth(upper_offset).filter(|c| *c == policy.c);

        match (lower_char, upper_char) {
            (Some(_), None) => true,
            (None, Some(_)) => true,
            _ => false,
        }
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
        let validator = Part2;
        let txt = include_str!("input.test.txt");
        let result = txt
            .lines()
            .map(|l| l.parse::<PasswordPolicy>().expect("password policy"))
            .filter(|p| (&validator).valid(p))
            .collect_vec();
        println!("{:?}", result);
        assert_eq!(1, result.len());
    }

    #[test]
    fn input_pt2_answer() {
        let test_input = include_str!("input.txt");
        assert_eq!(688, part2(test_input));
    }
}
