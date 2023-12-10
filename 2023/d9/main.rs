use std::{collections::HashMap, time::Instant, str::FromStr, i64};

use aoclib::{input, number};
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i64 {
    txt.lines()
        .map(|l| l.parse::<ReportEntry>().unwrap())
        .map(|re| re.descend())
        .sum()
} 

fn part2(txt: &str) -> i64 {
    txt.lines()
    .map(|l| l.parse::<ReportEntry>().unwrap())
    .map(|re: ReportEntry| re.descend_back())
    .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReportEntry(Vec<i64>);

impl ReportEntry {

    fn descend(self) -> i64 {
        let mut row = self.0;
        let mut down: Vec<i64> = Vec::new();

        down.push(*row.last().unwrap());

        loop {
            let row_itr = row.iter();
            let row_offset_itr = row.iter().skip(1);
            row = row_itr.zip(row_offset_itr)
                .map(|(&x, &y)| y - x)
                .collect_vec();

            down.push(*row.last().expect("sequence to finish before running out"));

            // if row.iter().any(|&i| i < 0) {
            //     panic!("{:?}", row);
            // }

            if row.iter().all(|&i| i == 0) {
                break;
            } else {
                // println!("{:?}", row);
            }
        }

        let mut increment = 0;
        while let Some(next) = down.pop() {
            increment += next;
        }

        increment
        
    }

    fn descend_back(self) -> i64 {
        let mut row = self.0;
        let mut down: Vec<i64> = Vec::new();

        down.push(*row.first().unwrap());

        loop {
            let row_itr = row.iter();
            let row_offset_itr = row.iter().skip(1);
            row = row_itr.zip(row_offset_itr)
                .map(|(&x, &y)| y - x)
                .collect_vec();

            down.push(*row.first().expect("sequence to finish before running out"));

            // if row.iter().any(|&i| i < 0) {
            //     panic!("{:?}", row);
            // }

            if row.iter().all(|&i| i == 0) {
                break;
            } else {
                // println!("{:?}", row);
            }
        }

        let mut increment = 0;
        while let Some(next) = down.pop() {
            // println!("{:?}", next);
            increment = next - increment;
            // println!("{:?}\n", increment);
        }

        increment
        
    }
}

impl FromStr for ReportEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(" ").map(|i| i.parse::<i64>().unwrap()).collect_vec()))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(114, part1(include_str!("input.test.txt")));
    }


    #[test]
    fn test_example_p2() {
        assert_eq!(2, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_vec_ordering() {
        let mut v = Vec::new();
        v.push(3);
        v.push(0);

        assert_eq!(0, v.pop().unwrap());
        assert_eq!(3, v.pop().unwrap());
    }

    #[test]
    fn test_single_example_pt1() {
        let entry = "10 13 16 21 30 45".parse::<ReportEntry>().unwrap();
        let result = entry.descend();
        assert_eq!(68, result);
    }

    #[test]
    fn test_single_example_pt2() {
        let entry = "10 13 16 21 30 45".parse::<ReportEntry>().unwrap();
        let result = entry.descend_back();
        assert_eq!(5, result);
    }
    
}
