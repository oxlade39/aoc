use std::{time::Instant, collections::HashSet};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let scanners = parse(include_str!("input.test2.txt"));

    let base = &scanners[0];
    let mut known_beacon_positions_in_basis: HashSet<Position> = HashSet::new();

    let base_sample = base.beacon_positions.0.iter().next().unwrap();
    let relative_sample = base.beacon_positions.in_basis(base_sample);

    let transforms = vec![
        DirectionalTransform { x: 1, y: 1, z: 1 },
        DirectionalTransform { x: 1, y: 1, z: -1 },
        DirectionalTransform { x: 1, y: -1, z: -1 },
        DirectionalTransform { x: -1, y: -1, z: -1 },

        DirectionalTransform { x: -1, y: 1, z: -1 },
        DirectionalTransform { x: -1, y: 1, z: 1 },
        DirectionalTransform { x: -1, y: -1, z: 1 },
    ];

    for secondary in scanners.iter().skip(1) {
        for t in transforms.iter() {
            let secondary_positions = secondary.beacon_positions.apply(t);
            let secondary_sample = secondary_positions.0.iter().next().unwrap();
            let relative_secondary = secondary_positions.in_basis(secondary_sample);
            
            let i: HashSet<_> = relative_secondary.0.intersection(&relative_sample.0).collect();
            if i.len() > 0 {
                println!("found intersection of {} through {:?}", i.len(), t);
            }
        }
    }

    // println!("known: {:?}", known_beacon_positions_in_basis);
    // println!("known len: {:?}", known_beacon_positions_in_basis.len());

    // let insersection_with_origin = known_beacon_positions_in_basis.intersection(
    //     &base.beacon_positions.0
    // );
    // println!("insersection with original\n{:?}", insersection_with_origin);
}

fn part2() {
}

fn parse(s: &str) -> Vec<Scanner> {
    let mut scanners: Vec<Scanner> = Vec::new();
    for line in s.lines() {
        if line.len() < 2 {
            continue;
        }
        if line.starts_with("--- ") {
            let scanner_id_parts: Vec<_> = line.split(" ").collect();
            let id_part = scanner_id_parts[2];
            let error = format!("expected integer: {}", id_part);
            let id = id_part.parse().expect(&error);
            scanners.push(Scanner::new(id));
        } else {
            let coords: Vec<_> = line.split(",").collect();
            let x: i64 = coords[0].parse().unwrap();
            let y: i64 = coords[1].parse().unwrap();
            let z = if coords.len() > 2 {
                coords[2].parse().unwrap()
            } else {
                0
            };
            let scanner = scanners.last_mut().unwrap();
            scanner.beacon_positions.insert(Position { x, y, z });
        }
    }
    scanners
}

#[derive(Debug, Clone)]
struct Scanner {
    id: i8,
    beacon_positions: BeaconPositions
}

#[derive(Debug, Clone)]
struct BeaconPositions(HashSet<Position>);

impl Scanner {
    fn new(id: i8) -> Scanner {
        Scanner{ id, beacon_positions: BeaconPositions::new() }
    }
}

impl BeaconPositions {
    fn new() -> Self {
        BeaconPositions(HashSet::new())
    }

    fn insert(&mut self, p: Position) {
        self.0.insert(p);
    }

    fn in_basis(&self, p: &Position) -> Self {
        let mut modified = Self::new();
        for pos in &self.0 {
            if pos != p {
                modified.insert(pos.in_basis(p));
            }
        }
        modified
    }

    fn apply<T>(&self, t: &T) -> BeaconPositions 
        where T: PositionTransform {

        let mut modified = Self::new();
        for p in &self.0 {
            modified.insert(t.transform(p));
        }
        modified
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64
}

impl Position {
    fn in_basis(&self, other: &Self) -> Self {
        Position {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    fn all_combinations(&self) -> HashSet<Self> {
        HashSet::from_iter(vec![
            Self{ x: self.x, y: self.y, z: self.z },
            Self{ x: self.y, y: self.z, z: self.x },
            Self{ x: self.z, y: self.x, z: self.y },
            Self{ x: self.x, y: self.z, z: self.y },

            Self{ x: -self.x, y: -self.y, z: -self.z },
            Self{ x: -self.y, y: -self.z, z: -self.x },
            Self{ x: -self.z, y: -self.x, z: -self.y },
            Self{ x: -self.x, y: -self.z, z: -self.y },

            Self{ x: self.x, y: -self.y, z: -self.z },
            Self{ x: self.y, y: -self.z, z: -self.x },
            Self{ x: self.z, y: -self.x, z: -self.y },
            Self{ x: self.x, y: -self.z, z: -self.y },

            Self{ x: self.x, y: self.y, z: -self.z },
            Self{ x: self.y, y: self.z, z: -self.x },
            Self{ x: self.z, y: self.x, z: -self.y },
            Self{ x: self.x, y: self.z, z: -self.y },

            Self{ x: -self.x, y: self.y, z: self.z },
            Self{ x: -self.y, y: self.z, z: self.x },
            Self{ x: -self.z, y: self.x, z: self.y },
            Self{ x: -self.x, y: self.z, z: self.y },

            Self{ x: self.x, y: -self.y, z: self.z },
            Self{ x: self.y, y: -self.z, z: self.x },
            Self{ x: self.z, y: -self.x, z: self.y },
            Self{ x: self.x, y: -self.z, z: self.y },
        ])
    }
}

trait PositionTransform {
    fn transform(&self, p: &Position) -> Position;
}

#[derive(Debug, Clone)]
struct DirectionalTransform {
    x: i8,
    y: i8,
    z: i8,
}

impl PositionTransform for DirectionalTransform {
    fn transform(&self, p: &Position) -> Position {
        Position {
            x: self.x as i64 * p.x,
            y: self.y as i64 * p.y,
            z: self.z as i64 * p.z,
        }
    }
}