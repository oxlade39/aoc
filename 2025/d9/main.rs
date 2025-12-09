use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let tiles: Vec<Point> = txt.lines().map(|l| l.parse::<Point>().unwrap()).collect();
    tiles.iter().combinations(2).map(|comb| {
        let a = comb[0];
        let b = comb[1];
        a.to_rect(b).area()
    }).max().expect("a max")
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rect {
    top_right: Point,
    bottom_left: Point,
}

impl Point {
    fn to_rect(&self, p: &Point) -> Rect {
        Rect { 
            top_right: Point { x: self.x.max(p.x), y: self.y.min(p.y) }, 
            bottom_left: Point { x: self.x.min(p.x), y: self.y.max(p.y) }
        }
    } 
}

impl Rect {
    fn area(&self) -> i64 {
        let width = (self.top_right.x - self.bottom_left.x) + 1;
        let height = (self.bottom_left.y - self.top_right.y) + 1;

        width * height
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xx, yy) = s.split_once(",").expect("sep");
        let x = xx.parse().unwrap();
        let y = yy.parse().unwrap();
        Ok(Point { x, y })
    }
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_area() {
        let a = Point { x: 2, y: 5};
        let b = Point { x: 9, y: 7};
        let area = a.to_rect(&b).area();
        assert_eq!(24, area);

        let a = Point { x: 7, y: 1};
        let b = Point { x: 11, y: 7};
        let area = a.to_rect(&b).area();
        assert_eq!(35, area);

        let a = Point { x: 7, y: 3};
        let b = Point { x: 2, y: 3};
        let area = a.to_rect(&b).area();
        assert_eq!(6, area);
    }

    #[test]
    fn test_parse() {
        let test_input = include_str!("input.test.txt");
        let points: Vec<_> = test_input.lines().map(|l| l.parse::<Point>().unwrap()).collect();
        for p in points {
            println!("{:?}", p);
        }        
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(50, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(4754955192, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
