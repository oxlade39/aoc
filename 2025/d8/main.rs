use core::str;
use std::{i64, time::Instant, usize};

use aoclib::timing;
use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    connections(txt, 1000 - 1)
}

fn part2(txt: &str) -> i64 {
    let all_points: Vec<_> = txt
        .lines()
        .map(|l| {
            let mut points = l.split(",");
            let x = points.next().expect("x").parse().unwrap();
            let y = points.next().expect("y").parse().unwrap();
            let z = points.next().expect("z").parse().unwrap();
            Position { x: x, y, z }
        })
        .collect();

    let ordered_distances: Vec<_> = all_points
        .iter()
        .combinations(2)
        .map(|combo| {
            let left = combo[0];
            let right = combo[1];
            left.distance(right)
        })
        .sorted_by(|a, b| a.squared_euclidean.cmp(&b.squared_euclidean))
        .collect();

    let mut all_circuits: Vec<Circuit> = Vec::new();

    for closest_pair in ordered_distances.into_iter() {
        let (left, right) = &closest_pair.positions;
        let removed_left = {
            if let Some((p, _)) = all_circuits
                .iter()
                .find_position(|c| c.junction_boxes.contains(left))
            {
                Some(all_circuits.remove(p))
            } else {
                None
            }
        };
        let removed_right = {
            if let Some((p, _)) = all_circuits
                .iter()
                .find_position(|c| c.junction_boxes.contains(right))
            {
                Some(all_circuits.remove(p))
            } else {
                None
            }
        };

        if removed_left.is_some() && removed_right.is_some() {
            // merge
            let mut rl = removed_left.unwrap();
            let rr = removed_right.unwrap();
            // println!("merge -> {:?} - {:?}\n{:?} - \n{:?}", left, right, rl, rr);
            rl.junction_boxes.extend(rr.junction_boxes);
            all_circuits.push(rl);
        } else if removed_left.is_some() {
            // add right to left circuit
            let mut rl = removed_left.unwrap();
            if rl.junction_boxes.insert(right.clone()) {
                // new connection
                // println!("found new left -> {:?} - {:?}", left, right);
            } else {
                // println!("found existing left -> {:?} - {:?}", left, right);
            }
            all_circuits.push(rl);
        } else if removed_right.is_some() {
            // add left to right circuit
            // println!("found new right -> {:?} - {:?}", left, right);
            let mut rr = removed_right.unwrap();
            rr.junction_boxes.insert(left.clone());
            all_circuits.push(rr);
        } else {
            // doesn't exist so create new
            // println!("add new -> {:?} - {:?}", left, right);
            all_circuits.push(Circuit::new(left.clone(), right.clone()));
        }

        if all_circuits[0].junction_boxes.len() == all_points.len() {
            return left.x * right.x;
        }
    }

    0
}

fn connections(txt: &str, pairs: usize) -> usize {
    let all_points: Vec<_> = txt
        .lines()
        .map(|l| {
            let mut points = l.split(",");
            let x = points.next().expect("x").parse().unwrap();
            let y = points.next().expect("y").parse().unwrap();
            let z = points.next().expect("z").parse().unwrap();
            Position { x: x, y, z }
        })
        .collect();

    let ordered_distances: Vec<_> = all_points
        .iter()
        .combinations(2)
        .map(|combo| {
            let left = combo[0];
            let right = combo[1];
            left.distance(right)
        })
        .sorted_by(|a, b| a.squared_euclidean.cmp(&b.squared_euclidean))
        .collect();

    let mut all_circuits: Vec<Circuit> = Vec::new();

    let mut connected_count = 0;
    for closest_pair in ordered_distances.into_iter() {
        let (left, right) = &closest_pair.positions;
        let removed_left = {
            if let Some((p, _)) = all_circuits
                .iter()
                .find_position(|c| c.junction_boxes.contains(left))
            {
                Some(all_circuits.remove(p))
            } else {
                None
            }
        };
        let removed_right = {
            if let Some((p, _)) = all_circuits
                .iter()
                .find_position(|c| c.junction_boxes.contains(right))
            {
                Some(all_circuits.remove(p))
            } else {
                None
            }
        };

        if removed_left.is_some() && removed_right.is_some() {
            // merge
            let mut rl = removed_left.unwrap();
            let rr = removed_right.unwrap();
            rl.junction_boxes.extend(rr.junction_boxes);
            all_circuits.push(rl);
        } else if removed_left.is_some() {
            // add right to left circuit
            let mut rl = removed_left.unwrap();
            if rl.junction_boxes.insert(right.clone()) {
                // new connection
            }
            all_circuits.push(rl);
        } else if removed_right.is_some() {
            // add left to right circuit
            let mut rr = removed_right.unwrap();
            rr.junction_boxes.insert(left.clone());
            all_circuits.push(rr);
        } else {
            // doesn't exist so create new
            all_circuits.push(Circuit::new(left.clone(), right.clone()));
        }

        connected_count += 1;

        if connected_count == pairs {
            break;
        }
    }

    report(&all_circuits)
}

fn report(all_circuits: &Vec<Circuit>) -> usize {
    let chosen: Vec<_> = all_circuits
        .iter()
        .sorted_by(|a, b| {
            let left = b.junction_boxes.len();
            let right = a.junction_boxes.len();
            left.cmp(&right)
        })
        .take(3)
        .collect();

    chosen.iter().map(|c| c.junction_boxes.len()).product()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    // fn in_basis(&self, other: &Self) -> Self {
    //     Translation::in_basis(other).transform(self)
    // }

    fn distance(&self, other: &Self) -> Distance {
        Distance {
            positions: (self.clone(), other.clone()),
            squared_euclidean: i64::pow(self.x - other.x, 2)
                + i64::pow(self.y - other.y, 2)
                + i64::pow(self.z - other.z, 2),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distance {
    positions: (Position, Position),
    squared_euclidean: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Circuit {
    junction_boxes: HashSet<Position>,
}

impl Circuit {
    fn new(left: Position, right: Position) -> Self {
        let mut junction_boxes = HashSet::new();
        junction_boxes.insert(left);
        junction_boxes.insert(right);
        Circuit { junction_boxes }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(40, connections(test_input, 10));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(42315, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(25272, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(8079278220, part2(test_input));
    }
}
