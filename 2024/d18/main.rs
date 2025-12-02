use core::str;
use std::{i64, time::Instant, usize};

use aoclib::{
    cartesian::{Plane, Point, Transform, Vector},
    distance::{Distance, ManhattenDistance},
    shortest_path::{Cost, Heuristic, Neighbours, astar},
    timing,
};
use hashbrown::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    min_steps(txt, 1024, Point::new(70, 70))
}

fn part2(txt: &str) -> String {
    first_blockage(txt, Point::new(70, 70))
}

fn min_steps(txt: &str, bytes: usize, end: Point) -> i64 {
    // invert y axis
    let end = Point::new(end.x, end.y * -1);

    let points: HashSet<_> = txt
        .lines()
        .take(bytes)
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let y: i64 = y.parse().unwrap();
            // invert axis because 0,0 is top,left (not bottom left)
            Point::new(x.parse().unwrap(), y * -1)
        })
        .collect();

    let start = Point::new(0, 0);
    let memory_space = Plane {
        top_left: start.clone(),
        bottom_right: end.clone(),
    };

    let state = Part1State {
        memory_space,
        corrupted: points,
    };

    let cost = UnitCost;

    let path = astar(
        &state,
        &cost,
        &Position(end.clone()),
        Position(start.clone()),
        |p| p.0 == end,
    );

    path.unwrap().total_cost
}

fn first_blockage(txt: &str, end: Point) -> String {
    // invert y axis
    let end = Point::new(end.x, end.y * -1);

    let points: Vec<_> = txt
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let y: i64 = y.parse().unwrap();
            // invert axis because 0,0 is top,left (not bottom left)
            Point::new(x.parse().unwrap(), y * -1)
        })
        .collect();

    let start = Point::new(0, 0);
    let memory_space = Plane {
        top_left: start.clone(),
        bottom_right: end.clone(),
    };

    let cost = UnitCost;
    // we could start at 1024 here but it makes the test harder and saving not significant
    let mut lower = 0;
    let mut upper = points.len();

    loop {
        // binary search starting from the mid point
        let bytes_pos = lower + (upper - lower) / 2;
        let corrupted: HashSet<_> = points[0..bytes_pos].iter().cloned().collect();
        let state = Part1State {
            memory_space: memory_space.clone(),
            corrupted: corrupted.clone(),
        };

        if upper - lower <= 1 {
            return format!("{},{}", points[bytes_pos].x, points[bytes_pos].y);
        }
        match astar(
            &state,
            &cost,
            &Position(end.clone()),
            Position(start.clone()),
            |p| p.0 == end,
        ) {
            Some(_) => {
                // if we find a path - update the lower bound to the tested point
                lower = bytes_pos;
            }
            _ => {
                // if we don't find a path - update the upper bound to the tested point
                upper = bytes_pos;
            }
        }
    }
}

struct Part1State {
    memory_space: Plane,
    corrupted: HashSet<Point>,
}

impl Neighbours<Position> for Part1State {
    fn neighbours(&self, state: &Position) -> Vec<Position> {
        let mut n = Vec::with_capacity(4);
        let left = state.0.transform(&Transform::left());
        if left.within(&self.memory_space) && !self.corrupted.contains(&left) {
            n.push(Position(left));
        }
        let right = state.0.transform(&Transform::right());
        if right.within(&self.memory_space) && !self.corrupted.contains(&right) {
            n.push(Position(right));
        }
        let up = state.0.transform(&Transform::up());
        if up.within(&self.memory_space) && !self.corrupted.contains(&up) {
            n.push(Position(up));
        }
        let down = state.0.transform(&Transform::down());
        if down.within(&self.memory_space) && !self.corrupted.contains(&down) {
            n.push(Position(down));
        }

        // println!("{:?} -> {:?}", state.0, n);

        n
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Position(Point);
struct UnitCost;

impl Cost<Position, i64> for UnitCost {
    fn measure(&self, _: &Position, _: &Position) -> i64 {
        1
    }
}

impl Heuristic<Position, i64> for Position {
    fn predict(&self, from: &Position) -> i64 {
        let v: Vector = (self.0.clone(), from.0.clone()).into();
        let ManhattenDistance(d) = ManhattenDistance::from_vector(v);
        d
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(22, min_steps(test_input, 12, Point::new(6, 6)));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(290, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!("6,-1", first_blockage(test_input, Point::new(6, 6)));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!("64,-54", part2(test_input));
    }
}
