use std::{str::FromStr, time::Instant};

use aoclib::cartesian::{Point, Transform};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i64 {
    let dp: Pt1DigPlan = txt.parse().unwrap();
    solve(dp.instructions)
}

fn part2(txt: &str) -> i64 {
    let dp: Pt2DigPlan = txt.parse().unwrap();
    solve(dp.instructions)
}

fn solve(instructions: Vec<Instruction>) -> i64 {
    let mut last_point: Point = (0, 0).into();
    let mut points: Vec<Point> = vec![last_point.clone()];
    for i in instructions {
        last_point = last_point.transform(&i.into());
        points.push(last_point.clone());
    }
    Polygon(points).area()
}

#[derive(Debug)]
struct Pt1DigPlan {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Pt2DigPlan {
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    count: usize,
}

impl From<Instruction> for Transform {
    fn from(value: Instruction) -> Self {
        let dir: Transform = value.direction.into();
        (dir * value.count as i64).into()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for Transform {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
            Direction::Up => (0, 1).into(),
            Direction::Down => (0, -1).into(),
        }
    }
}

impl FromStr for Pt1DigPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<_> = s
            .lines()
            .map(|l| {
                let parts: Vec<_> = l.split(" ").collect();
                let direction = match parts[0] {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => panic!("bad dir"),
                };
                let num = parts[1].parse::<usize>().unwrap();
                Instruction {
                    direction,
                    count: num,
                }
            })
            .collect();
        Ok(Pt1DigPlan { instructions })
    }
}

impl FromStr for Pt2DigPlan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<_> = s
            .lines()
            .map(|l| {
                let parts: Vec<_> = l.split(" ").collect();
                let count = i64::from_str_radix(&parts[2][2..7], 16).unwrap();
                let direction = match i64::from_str_radix(&parts[2][7..8], 16) {
                    Ok(0) => Direction::Right,
                    Ok(1) => Direction::Down,
                    Ok(2) => Direction::Left,
                    Ok(3) => Direction::Up,
                    _ => panic!("bad dir"),
                };
                Instruction {
                    direction,
                    count: count as usize,
                }
            })
            .collect();
        Ok(Pt2DigPlan { instructions })
    }
}

struct Polygon(Vec<Point>);

impl Polygon {
    fn area(&self) -> i64 {
        let first = self.0.iter().next().unwrap();
        let (area, perim, _) =
            self.0
                .iter()
                .skip(1)
                .fold((0, 0, first), |(area, perimeter, point), next_point| {
                    let next_area = area + ((point.x * next_point.y) - (point.y * next_point.x));
                    let next_perim =
                        perimeter + point.x.abs_diff(next_point.x) + point.y.abs_diff(next_point.y);
                    (next_area, next_perim, next_point)
                });

        (area.abs() + perim as i64) / 2 + 1
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(62, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(952408144115, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let txt = include_str!("input.test.txt");
        let dp: Pt1DigPlan = txt.parse().unwrap();
        assert_eq!(Direction::Right, dp.instructions[0].direction);
        assert_eq!(Direction::Down, dp.instructions[1].direction);
        assert_eq!(Direction::Left, dp.instructions[2].direction);
    }
}
