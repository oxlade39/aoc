#![allow(dead_code)]
use std::{time::Instant, collections::{HashSet, HashMap}, hash::Hash};

use itertools::Itertools;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let scanners = parse(include_str!("input.txt"));

    let distances = calculate_distances(&scanners);
    let mut path = sort_distances(&distances);

    let mut positions: HashMap<i8, Vec<Position>> = HashMap::new();
    loop {
        if path.is_empty() {
            break;
        }

        let (target_id, source_id, target_distances, source_distances) = path.pop().unwrap();
        let (rot, trans) = find_shared_translation(&target_distances, &source_distances).unwrap();
        let source = &scanners[source_id as usize];

        let transformed = match positions.remove(&source_id) {
            Some(mut existing_from) => {
                existing_from.extend(source.beacon_positions.0.clone());
                BeaconPositions(existing_from)
                    .apply(&rot)
                    .apply(&trans)
                    .0
            },
            _ => source.beacon_positions
                    .apply(&rot)
                    .apply(&trans)
                    .0
        };

        if let Some(existing) = positions.get_mut(&target_id) {
            existing.extend(transformed);
        } else {
            positions.insert(target_id, transformed);
        }

    }
    if positions.keys().len() > 1 {
        panic!("too many remaining keys: {:?}", positions.keys());
    }
    let incl_dupes = positions.remove(&0).unwrap();
    let unique: HashSet<Position> = HashSet::from_iter(incl_dupes);    
    println!("part1: {}", unique.len());

}

fn part2() {
    let scanners = parse(include_str!("input.txt"));

    let distances = calculate_distances(&scanners);
    let mut path = sort_distances(&distances);
    let mut positions: HashMap<i8, Vec<Position>> = HashMap::new();

    loop {
        if let Some((target_id, source_id, target_distances, source_distances)) = path.pop() {
            let (rot, trans) = find_shared_translation(
                target_distances, 
                source_distances
            ).unwrap();

            let scanner = trans.transform(&(0,0,0).into());

            let transformed = match positions.remove(&source_id) {
                Some(existing_from) => {
                    let mut bp = BeaconPositions(existing_from)
                        .apply(&rot)
                        .apply(&trans)
                        .0;
                    bp.push(scanner);
                    bp
                },
                _ => vec![scanner]
            };
    
            if let Some(existing) = positions.get_mut(&target_id) {
                existing.extend(transformed);
            } else {
                positions.insert(target_id, transformed);
            }

        } else {
            break;
        }
    }
    if positions.keys().len() > 1 {
        panic!("too many remaining keys: {:?}", positions.keys());
    }
    
    let result = positions.get(&0).unwrap().iter()
        .combinations(2)
        .map(|combo| {
            let x_delta = i64::abs(combo[0].x - combo[1].x);
            let y_delta = i64::abs(combo[0].y - combo[1].y);
            let z_delta = i64::abs(combo[0].z - combo[1].z);
            x_delta + y_delta + z_delta
        })
        .max()
        .unwrap();
    println!("pt2: {}", result);
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

fn ncr(n: i64, r: i64) -> i64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn factorial(n: i64) -> i64 {
    match n  {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n
    }
}

fn calculate_distances(scanners: &Vec<Scanner>) -> Vec<(i8, i8, Vec<Distance>, Vec<Distance>)> {
    let mut distances: Vec<(i8, i8, Vec<Distance>, Vec<Distance>)> = vec![];
    for combo in scanners.iter().combinations(2) {
        let left = combo[0];
        let right = combo[1];

        let left_distances: Vec<_> = left.beacon_positions.0.iter()
            .combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .collect();

        let right_distances: Vec<_> = right.beacon_positions.0.iter()
            .combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .collect();

        let left_euclidean: HashSet<_> = HashSet::from_iter(left_distances.iter().map(|d|d.squared_euclidean));
        let right_euclidean: HashSet<_> = HashSet::from_iter(right_distances.iter().map(|d|d.squared_euclidean));
        
        let shared_distances = left_euclidean
            .intersection(&right_euclidean)
            .count();

        if shared_distances as i64 >= ncr(12, 2) {
            distances.push((left.id, right.id, left_distances, right_distances));
        }
    }
    distances
}

/// build a graph of overlapping scanners
/// the graph can be iterated in reverse to perform necessary translations
fn sort_distances(distances: &Vec<(i8, i8, Vec<Distance>, Vec<Distance>)>) -> Vec<(i8, i8, &Vec<Distance>, &Vec<Distance>)> {    
    let mut path = Vec::new();
    let mut visited = HashSet::new();
    visit_path(&distances, &mut path, &mut visited, (-1, 0));
    path
}

fn find_shared_translation(left_distances: &Vec<Distance>, right_distances: &Vec<Distance>) -> Option<(Rotation, Translation)> {
    for left_dist in left_distances {
        let (left_0, left_1) = &left_dist.positions;
        for right_dist in right_distances {
            let (right_0, right_1) = &right_dist.positions;

            // if two points in left scanner have the same distance as two points in right scanner
            // look for the rotation in right that aligns the axis
            if left_dist.squared_euclidean == right_dist.squared_euclidean {
                for rot in all_rotations() {
                    let rot_0 = rot.transform(right_0);
                    let rot_1 = rot.transform(right_1);
                    let x_rot_delta = rot_0.x - rot_1.x;
                    let y_rot_delta = rot_0.y - rot_1.y;
                    let z_rot_delta = rot_0.z - rot_1.z;
                    let x_delta = left_0.x - left_1.x;
                    let y_delta = left_0.y - left_1.y;
                    let z_delta = left_0.z - left_1.z;
    
                    // check the distances are equal along all axis
                    if i64::abs(x_rot_delta) == i64::abs(x_delta) && 
                        i64::abs(y_rot_delta) == i64::abs(y_delta) && 
                        i64::abs(z_rot_delta) == i64::abs(z_delta) && 
                        i64::signum(x_rot_delta) == i64::signum(x_delta) && 
                        i64::signum(y_rot_delta) == i64::signum(y_delta) && 
                        i64::signum(y_rot_delta) == i64::signum(y_delta) {
                        
                        let diff_0 = rot_0.difference(left_0);
                        let diff_3 = rot_1.difference(left_1);
    
                        // if the 2 right points share the same translation back to left
                        // then they are aligned with this rotation
                        if diff_0 == diff_3 {
                            let d = left_0.difference(&rot_0);
                            // right scanner position is at `d.transform(&(0,0,0).into())`
                            return Some((rot, d));
                        }
                    }
                }
            }
        }
    }
    None
}

fn all_rotations() -> HashSet<Rotation> {
    let start_positions: Vec<Position> = vec![
        (1, 2, 3).into(),   // reference position - right hand, Index Finger forward, mf left, thumb up
        (2, -1, 3).into(),  // I.F left
        (-2, 1, 3).into(),  // I.F right
        (-1, -2, 3).into(), // I.F back
        (3, 2, -1).into(),  // I.F up
        (-3, 2, 1).into()   // I.F down
    ];
    let mut unique: HashSet<Rotation> = HashSet::new();

    for p in start_positions {
        let mut p = p.clone();
        for _ in 0..4 {
            unique.insert(Rotation([p.x as i8, p.y as i8, p.z as i8]));    
            p = p.rotate_90(Axis::X);
        }
    }
    unique
}

#[derive(Debug, Clone)]
struct Scanner {
    id: i8,
    beacon_positions: BeaconPositions
}

#[derive(Debug, Clone)]
struct BeaconPositions(Vec<Position>);

impl Scanner {
    fn new(id: i8) -> Scanner {
        Scanner{ id, beacon_positions: BeaconPositions::new() }
    }
}

impl BeaconPositions {
    fn new() -> Self {
        BeaconPositions(Vec::new())
    }

    fn insert(&mut self, p: Position) {
        self.0.push(p);
    }

    fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
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

    fn distances(&self) -> Vec<Distance> {
        let mut result = Vec::new();

        for pairs in self.0.iter().combinations(2) {
            let distance = pairs[0].distance(pairs[1]);
            result.push(distance);
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64
}

enum Axis {
    X,
    Y,
    Z
}

impl Position {
    fn in_basis(&self, other: &Self) -> Self {
        Translation::in_basis(other).transform(self)
    }

    fn distance(&self, other: &Self) -> Distance {
        Distance { 
            positions: (self.clone(), other.clone()),
            squared_euclidean: i64::pow(self.x - other.x, 2) + i64::pow(self.y - other.y, 2) + i64::pow(self.z - other.z, 2)
        }
    }

    fn rotate_90(&self, axis: Axis) -> Self {
        match axis {
            Axis::X => Rotation([1, 3, -2]).transform(self),
            Axis::Y => Rotation([3, 2, -1]).transform(self),
            Axis::Z => Rotation([2, -1, 3]).transform(self),
        }
    }

    fn reflect(&self, axis: Axis) -> Self {
        match axis {
            Axis::X => Translation([-self.x * 2, 0, 0]).transform(self),
            Axis::Y => Translation([0, -self.y * 2, 0]).transform(self),
            Axis::Z => Translation([0, 0, -self.z * 2]).transform(self),
        }
    }

    fn difference(&self, other: &Position) -> Translation {
        Translation([
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        ])
    }
}

impl From<(i64, i64, i64)> for Position {
    fn from(coords: (i64, i64, i64)) -> Self {
        Position { x: coords.0, y: coords.1, z: coords.2 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Distance {
    positions: (Position, Position),
    squared_euclidean: i64,
}

trait PositionTransform {
    fn transform(&self, p: &Position) -> Position;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Rotation([i8; 3]);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Translation([i64; 3]);

impl PositionTransform for Rotation {
    fn transform(&self, p: &Position) -> Position {
        let coords = [
            p.x,
            p.y,
            p.z
        ];
        let signs = [i8::signum(self.0[0]), i8::signum(self.0[1]), i8::signum(self.0[2])];
        let axis = [i8::abs(self.0[0]) as usize, i8::abs(self.0[1]) as usize, i8::abs(self.0[2]) as usize];

        let x = coords[axis[0] - 1] * signs[0] as i64;
        let y = coords[axis[1] - 1] * signs[1] as i64;
        let z = coords[axis[2] - 1] * signs[2] as i64;
        Position{ x, y, z }
    }
}

impl PositionTransform for Translation {
    fn transform(&self, p: &Position) -> Position {
        Position {
            x: self.0[0] + p.x,
            y: self.0[1] + p.y,
            z: self.0[2] + p.z
        }
    }
}

impl Translation {
    fn in_basis(p: &Position) -> Translation {
        Translation([-p.x, -p.y, -p.z])
    }
}

#[test]
fn test_transorm_combinations() {
    let t = all_rotations();
    assert_eq!(24, t.len());

    let mut dupe_test: HashSet<[i8; 3]> = HashSet::new();
    for trans in &t {
        dupe_test.insert(trans.0);
    }
    assert_eq!(24, dupe_test.len());
}

#[test]
fn test_point_transforms() {
    let position = Position{ x: 5, y: 6, z: -4 };
    let t = Rotation([1, 2, 3]);
    let result = t.transform(&position);

    assert_eq!(result, Position{ x: 5, y: 6, z: -4 });

    let t = Rotation([-1, -2, -3]);
    let result = t.transform(&position);

    assert_eq!(result, Position{ x: -5, y: -6, z: 4 });

    let t = Rotation([-3, -1, -2]);
    let result = t.transform(&position);

    assert_eq!(result, Position{ x: 4, y: -5, z: -6 });
}

#[test]
fn test_perform_transforms() {
    let position = Position{ x: 5, y: 6, z: -4};
    let results: Vec<Position> = all_rotations().iter().map(|t| t.transform(&position)).collect();

    for p in results {
        println!("{},{},{}", p.x, p.y, p.z);
    }
}

#[test]
fn test_rotate_x_position() {
    let p = Position { x: 1, y: 2, z: 3 };
    let rotated = p.rotate_90(Axis::X);
    assert_eq!(rotated, Position { x: 1, y: 3, z: -2 });
    let rotated = rotated.rotate_90(Axis::X);
    assert_eq!(rotated, Position { x: 1, y: -2, z: -3 });
    let rotated = rotated.rotate_90(Axis::X);
    assert_eq!(rotated, Position { x: 1, y: -3, z: 2 });
    let rotated = rotated.rotate_90(Axis::X);
    assert_eq!(rotated, p);
}

#[test]
fn test_rotate_y_position() {
    let p = Position { x: 1, y: 2, z: 3 };
    let rotated = p.rotate_90(Axis::Y);
    assert_eq!(rotated, Position { x: 3, y: 2, z: -1 });
    let rotated = rotated.rotate_90(Axis::Y);
    assert_eq!(rotated, Position { x: -1, y: 2, z: -3 });
    let rotated = rotated.rotate_90(Axis::Y);
    assert_eq!(rotated, Position { x: -3, y: 2, z: 1 });
    let rotated = rotated.rotate_90(Axis::Y);
    assert_eq!(rotated, p);
}

#[test]
fn test_rotate_z_position() {
    let p = Position { x: 1, y: 2, z: 3 };
    let rotated = p.rotate_90(Axis::Z);
    assert_eq!(rotated, Position { x: 2, y: -1, z: 3 });
    let rotated = rotated.rotate_90(Axis::Z);
    assert_eq!(rotated, Position { x: -1, y: -2, z: 3 });
    let rotated = rotated.rotate_90(Axis::Z);
    assert_eq!(rotated, Position { x: -2, y: 1, z: 3 });
    let rotated = rotated.rotate_90(Axis::Z);
    assert_eq!(rotated, p);
}

#[test]
fn test_all_rotations2() {
    let start_positions: Vec<Position> = vec![
        (1, 2, 3).into(),   // reference position - right hand, index finger forward, mf left, thumb up
        (2, -1, 3).into(),  // I.F left
        (-2, 1, 3).into(),  // I.F right
        (-1, -2, 3).into(), // I.F back
        (3, 2, -1).into(),  // I.F up
        (-3, 2, 1).into()   // I.F down
    ];
    let mut unique: HashSet<Position> = HashSet::new();

    for p in start_positions {
        let mut p = p.clone();
        for _ in 0..4 {
            unique.insert(p.clone());    
            p = p.rotate_90(Axis::X);
        }
    }

    let all_points: HashSet<Position> = HashSet::from_iter(vec![
        ( 1,  2,  3).into(),
        ( 2, -1,  3).into(),
        (-1, -2,  3).into(),
        (-2,  1,  3).into(),
        (-3,  2,  1).into(),
        ( 2,  3,  1).into(),
        ( 3, -2,  1).into(),
        (-2, -3,  1).into(),
        (-1,  2, -3).into(),
        ( 2,  1, -3).into(),
        ( 1, -2, -3).into(),
        (-2, -1, -3).into(),
        ( 3,  2, -1).into(),
        ( 2, -3, -1).into(),
        (-3, -2, -1).into(),
        (-2,  3, -1).into(),
        ( 1, -3,  2).into(),
        (-3, -1,  2).into(),
        (-1,  3,  2).into(),
        ( 3,  1,  2).into(),
        ( 1,  3, -2).into(),
        ( 3, -1, -2).into(),
        (-1, -3, -2).into(),
        (-3,  1, -2).into(),
    ]);
    assert_eq!(all_points, unique);
}

#[test]
fn test_reflection() {
    let a = Position { x: 1, y: 1, z: 1 };
    assert_eq!(a.reflect(Axis::X), Position { x: -1, y: 1, z: 1 })
}

#[test]
fn test_position_basis_transforms() {
    let a = Position { x: 10, y: 12, z: 5 };
    let b = Position { x: 1, y: 1, z: 1 };

    let transformed = b.in_basis(&a); // b - a = t
    assert_eq!(transformed, Position { x: -9, y: -11, z: -4 });

    let neg_a = a
        .reflect(Axis::X)
        .reflect(Axis::Y)
        .reflect(Axis::Z);
    let reversed = transformed.in_basis(&neg_a); // t - -a = b
    assert_eq!(reversed, b);
}

#[test]
fn test_differece_translation() {
    let p = Position { x: 10, y: 15, z: 20 };
    let p_2 = Position { x: 15, y: 20, z: 25 };

    let t = p.difference(&p_2);
    println!("{:?} -> {:?} = {:?}", p_2, p, t);
    assert_eq!(t.transform(&p_2), p);
}

#[test]
fn test_simple_positioning() {
    let scanners: [Position; 2] = [
        (0, 0, 0).into(),
        (1, -7, 0).into()
    ];

    let scanner0_positions: [Position; 3] = [
        (1,0,0).into(),
        (2,0,0).into(),
        (2,-3,0).into(),
    ];

    let scanner1_positions: [Position; 3] = [
        (7,0,0).into(),
        (7,-1,0).into(),
        (4,-1,0).into(),
    ];

    let scanner0_basis1: Vec<_> = scanner0_positions.iter().map(|p| p.in_basis(&scanner0_positions[0])).collect();
    let scanner1_basis1: Vec<_> = scanner1_positions.iter().map(|p| p.in_basis(&scanner1_positions[0])).collect();

    assert_eq!(
        scanner0_basis1,
        vec![
            (0,0,0).into(),
            (1,0,0).into(),
            (1,-3,0).into(),
        ]
    );
    assert_eq!(
        scanner1_basis1,
        vec![
            (0,0,0).into(),
            (0,-1,0).into(),
            (-3,-1,0).into(),
        ]
    );

    let rotation = Rotation([-2, 1, 3]);

    let x = rotation.transform(&(0, -1, 0).into());
    println!("x: {:?}", x);

    let rotated = rotation.transform(&scanner1_positions[0]);
    assert_eq!(rotated, (0,7,0).into());
    let translation = scanner0_positions[0].difference(&rotated);
    let scanner1_calculated_pos = translation.transform(&(0,0,0).into());
    assert_eq!(scanners[1], scanner1_calculated_pos);
}

#[test]
fn test_scanner_positioning() {
    let scanner_0_points: [Position; 2] = [
        (-618,-824,-621).into(),
        (-537,-823,-458).into(),
    ];
    let scanner_1_points: [Position; 2] = [
        (686,422,578).into(),
        (605,423,415).into(),
    ];
    let _scanner_1: Position = (68,-1246,-43).into();

    let scanner_0_distance = scanner_0_points[0].distance(&scanner_0_points[1]);
    let scanner_1_distance = scanner_1_points[0].distance(&scanner_1_points[1]);

    assert_eq!(scanner_0_distance.squared_euclidean, scanner_1_distance.squared_euclidean);
}

fn visit_path<'a>(
    pairs: &'a Vec<(i8, i8, Vec<Distance>, Vec<Distance>)>,
    path: &mut Vec<(i8, i8, &'a Vec<Distance>, &'a Vec<Distance>)>,
    visited: &mut HashSet<i8>,
    pair: (i8, i8)
) {
    for (left, right, left_d, right_d) in pairs.iter().filter(|(left, _right, _, _)|*left == pair.1) {
        if visited.contains(right) == false && (*right, *left) != pair {
            path.push((*left, *right, left_d, right_d));
            visited.insert(*right);
            visit_path(pairs, path, visited, (*left, *right));
        }
    }

    for (right, left, right_d, left_d) in pairs.iter().filter(|(_right, left, _, _)|*left == pair.1) {
        if visited.contains(right) == false && (*right, *left) != pair {
            path.push((*left, *right, left_d, right_d));
            visited.insert(*right);
            visit_path(pairs, path, visited, (*left, *right));
        }
    }
}