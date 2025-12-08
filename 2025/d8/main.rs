use core::str;
use std::{collections::BinaryHeap, i64, ops::Index, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};
use hashbrown::{HashMap, HashSet};
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

    println!("{} pairs", ordered_distances.len());

    let mut all_circuits: Vec<Circuit> = Vec::new();

    let mut connected_count = 0;
    for closest_pair in ordered_distances.into_iter() {
        let (left, right) = &closest_pair.positions;
        // println!("closest: {:?} - {:?}", left, right);

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
            println!("merge -> {:?} - {:?}\n{:?} - \n{:?}", left, right, rl, rr);
            rl.junction_boxes.extend(rr.junction_boxes);
            all_circuits.push(rl);
        } else if removed_left.is_some() {
            // add right to left circuit
            let mut rl = removed_left.unwrap();
            if rl.junction_boxes.insert(right.clone()) {
                // new connection
                println!("found new left -> {:?} - {:?}", left, right);
            } else {
                println!("found existing left -> {:?} - {:?}", left, right);
            }
            all_circuits.push(rl);
        } else if removed_right.is_some() {
            // add left to right circuit
            println!("found new right -> {:?} - {:?}", left, right);
            let mut rr = removed_right.unwrap();
            rr.junction_boxes.insert(left.clone());
            all_circuits.push(rr);
        } else {
            // doesn't exist so create new
            println!("add new -> {:?} - {:?}", left, right);
            all_circuits.push(Circuit::new(left.clone(), right.clone()));
        }

        // report(&all_circuits);
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

    for item in &chosen {
        println!("[{}]", item.junction_boxes.len());
    }

    chosen.iter().map(|c| c.junction_boxes.len()).product()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

enum Axis {
    X,
    Y,
    Z,
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

    // fn rotate_90(&self, axis: Axis) -> Self {
    //     match axis {
    //         Axis::X => Rotation([1, 3, -2]).transform(self),
    //         Axis::Y => Rotation([3, 2, -1]).transform(self),
    //         Axis::Z => Rotation([2, -1, 3]).transform(self),
    //     }
    // }

    // fn reflect(&self, axis: Axis) -> Self {
    //     match axis {
    //         Axis::X => Translation([-self.x * 2, 0, 0]).transform(self),
    //         Axis::Y => Translation([0, -self.y * 2, 0]).transform(self),
    //         Axis::Z => Translation([0, 0, -self.z * 2]).transform(self),
    //     }
    // }

    fn difference(&self, other: &Position) -> Translation {
        Translation([self.x - other.x, self.y - other.y, self.z - other.z])
    }
}

impl From<(i64, i64, i64)> for Position {
    fn from(coords: (i64, i64, i64)) -> Self {
        Position {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distance {
    positions: (Position, Position),
    squared_euclidean: i64,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.squared_euclidean.partial_cmp(&self.squared_euclidean)
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.squared_euclidean.cmp(&self.squared_euclidean)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Translation([i64; 3]);

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

    fn add_pair(&mut self, left: Position, right: Position) {
        self.junction_boxes.insert(left);
        self.junction_boxes.insert(right);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_ordering_by_distance() {
        let txt = include_str!("input.test.txt");
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

        for item in ordered_distances {
            println!("{:?} -> \t[{}]", item.positions, item.squared_euclidean);
        }
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(40, connections(test_input, 10));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
        assert!(part1(test_input) > 6804);
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
