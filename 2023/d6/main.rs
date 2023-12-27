use std::{str::FromStr, time::Instant};

use itertools::Itertools;
use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i64 {
    let input: Part1Input = txt.parse().expect("input");
    input
        .0
        .iter()
        .map(|race| race.improvements().count() as i64)
        .product()
}

fn part2(txt: &str) -> i64 {
    let input: Part2Input = txt.parse().expect("input");
    input.0.improvements().count() as i64
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Time(i64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct RecordDistance(i64);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Race {
    time: Time,
    distance: RecordDistance,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Improvement(Race);

impl Race {
    fn new(time: Time, distance: RecordDistance) -> Self {
        Race { time, distance }
    }

    fn improvements(&self) -> impl Iterator<Item = Improvement> + '_ {
        (0..self.time.0)
            .map(|i| {
                let d = (self.time.0 - i) * i;
                Improvement(Race::new(Time(i), RecordDistance(d)))
            })
            .filter(|i| i.0.distance.0 > self.distance.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Part1Input(Vec<Race>);

#[derive(Debug, PartialEq, Eq)]
struct Part2Input(Race);

impl FromStr for Part1Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.lines().collect_vec();

        let time_part = parts[0].split(":").skip(1).next().unwrap();
        let distance_part = parts[1].split(":").skip(1).next().unwrap();

        let re = regex::Regex::new(r"\d+").unwrap();

        let times = re
            .find_iter(time_part)
            .map(|mat| Time(mat.as_str().parse().expect("time")));

        let distances = re
            .find_iter(distance_part)
            .map(|mat| RecordDistance(mat.as_str().parse().expect("distance")));

        let it: Vec<_> = zip(times, distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok(Part1Input(it))
    }
}

impl FromStr for Part2Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.lines().collect_vec();

        let time_part = parts[0].split(":").skip(1).next().unwrap();
        let distance_part = parts[1].split(":").skip(1).next().unwrap();

        let time: i64 = time_part.replace(" ", "").parse().expect("time");
        let distance: i64 = distance_part.replace(" ", "").parse().expect("distance");

        Ok(Part2Input(Race::new(Time(time), RecordDistance(distance))))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(288, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(71503, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_regexp() {
        let re = regex::Regex::new(r"\d+").unwrap();
        let input = "      7  15   30";

        let items: Vec<_> = re.find_iter(input).map(|mat| mat.as_str()).collect();
        assert_eq!(vec!["7", "15", "30"], items);
    }

    #[test]
    fn test_parse() {
        let input = include_str!("input.test.txt");

        let expected = Part1Input(vec![
            Race::new(Time(7), RecordDistance(9)),
            Race::new(Time(15), RecordDistance(40)),
            Race::new(Time(30), RecordDistance(200)),
        ]);
        let actual = input.parse().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_distances() {
        let r = Race::new(Time(7), RecordDistance(9));
        let d: Vec<_> = r.improvements().collect();

        assert_eq!(
            vec![
                Improvement(Race::new(Time(2), RecordDistance(10))),
                Improvement(Race::new(Time(3), RecordDistance(12))),
                Improvement(Race::new(Time(4), RecordDistance(12))),
                Improvement(Race::new(Time(5), RecordDistance(10))),
            ],
            d
        );
    }
}
