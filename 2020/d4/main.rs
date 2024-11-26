use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    str::FromStr,
    time::Instant,
};

use aoclib::input::{empty_line_chunks, NEW_LINE};
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
    empty_line_chunks(txt)
        .filter(|chunk| !chunk.is_empty())
        .filter_map(|chunk| chunk.parse::<Passport>().ok())
        .count()
}

fn part2(txt: &str) -> usize {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Passport {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID)
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: Height,
    hair_color: String,
    eye_color: String,
    passpord_id: PassportId,
    country_id: Option<usize>,
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(NEW_LINE, " ");
        let mut key_values: HashMap<_, _> = s
            .split(" ")
            .map(|key_pair| {
                key_pair
                    .split(":")
                    .collect_tuple::<(&str, &str)>()
                    .expect(&format!("tuple but was '{}' in '{}'", key_pair, s))
            })
            .collect();

        let required_keys: HashSet<&str> =
            HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

        let diff: HashSet<_> = required_keys.iter()
            .filter(|required| !key_values.contains_key(*required))
            .collect();

        if diff.is_empty() {
            Ok(Passport {
                birth_year: key_values.remove("byr").unwrap().parse().expect("integer"),
                issue_year: key_values.remove("iyr").unwrap().parse().expect("integer"),
                expiration_year: key_values.remove("eyr").unwrap().parse().expect("integer"),
                height: key_values.remove("hgt").unwrap().parse::<Height>().unwrap(),
                hair_color: key_values.remove("hcl").unwrap().to_owned(),
                eye_color: key_values.remove("ecl").unwrap().to_owned(),
                passpord_id: key_values.remove("pid").unwrap().parse().unwrap(),
                country_id: key_values.remove("cid").map(|i| i.parse().expect("integer")),
            })
        } else {
            Err(format!("required fields missing: [{:?}]", diff))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Height {
    Length(Length),
    Colour(String)
}

#[derive(Debug, PartialEq, Eq)]
enum Length {
    Inches(usize),
    Centimetres(usize),
    Unknown(usize),
}

impl FromStr for Length {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let unit = &s[(s.len() - 2)..(s.len())];
        match unit {
            "cm" => Ok(Length::Centimetres(s[..s.len() - 2].parse().unwrap())),
            "in" => Ok(Length::Inches(s[..s.len() - 2].parse().unwrap())),
            _ => Ok(Length::Unknown(s.parse().expect(&format!("'{}'", s)))),
        }
    }
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..1] == "#" {
            Ok(Height::Colour(s.to_owned()))
        } else {
            s.parse::<Length>().map(Height::Length)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PassportId {
    Standard(usize),
    Length(Length),
    Colour(String),
}

impl FromStr for PassportId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if &s[0..1] == "#" {
            return Ok(PassportId::Colour(s.to_owned()));
        }
        match s.parse::<usize>() {
            Ok(n) => Ok(PassportId::Standard(n)),
            Err(_) =>  match s.parse::<Length>() {
                Ok(l) => Ok(PassportId::Length(l)),
                Err(e) => Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_length() {
        assert_eq!(Length::Centimetres(123), "123cm".parse().unwrap());
        assert_eq!(Length::Inches(13), "13in".parse().unwrap());
    }

    #[test]
    fn parse_passport() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        let p: Passport = input.parse().unwrap();
        assert_eq!(p, Passport {
            eye_color: "gry".to_owned(),
            passpord_id: PassportId::Standard(860033327),
            expiration_year: 2020,
            hair_color: "#fffffd".to_owned(),
            birth_year: 1937,
            issue_year: 2017,
            country_id: Some(147),
            height: Height::Length(Length::Centimetres(183)),
        });
    }

    #[test]
    fn parse_example() {
        let test_input = include_str!("input.test.txt");
        let all: Vec<Result<Passport, String>> = empty_line_chunks(&test_input).map(|l| l.parse()).collect();
        assert!(all[0].is_ok());
        assert_eq!(all[1], Err("required fields missing: [{\"hgt\"}]".to_owned()));
        assert!(all[2].is_ok());
        assert_eq!(all[3], Err("required fields missing: [{\"byr\"}]".to_owned()));
    }

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(204, part1(test_input));
    }

    #[test]
    fn sample_input_pt2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
