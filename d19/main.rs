use std::{time::Instant, collections::{HashSet, HashMap}};

use itertools::Itertools;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let scanners = parse(include_str!("input.test2.txt"));

    let mut right_nodes: HashSet<usize> = HashSet::from_iter([0]);

    let mut scanner_pairs: Vec<(usize, usize)> = Vec::new();
    let threshold = nCr(12, 2);
    println!("threshold: {}", threshold);
    for scanner in (0..scanners.len()).combinations(2) {
        let left = &scanners[scanner[0]];
        let right = &scanners[scanner[1]];

        let left_dist: HashSet<_> = left.beacon_positions.distances().iter().map(|d| d.squared_euclidean).collect();
        let right_dist: HashSet<_> = right.beacon_positions.distances().iter().map(|d| d.squared_euclidean).collect();
        let shared_dist = left_dist.intersection(&right_dist).count();
        if shared_dist as i64 >= threshold {

            if right_nodes.contains(&scanner[1]) {
                right_nodes.insert(scanner[0]);
                scanner_pairs.push((scanner[1], scanner[0]));
            } else {
                right_nodes.insert(scanner[1]);
                scanner_pairs.push((scanner[0], scanner[1]));
            }
        }
    }
    println!("scanner pairs: {:?}", scanner_pairs);

    let rotations = all_rotations();
    let mut scanner_transforms: Vec<(Rotation, Translation)> = Vec::new();

    for (left_idx, right_idx) in &scanner_pairs {
        let left = &scanners[left_idx.clone()];
        let left_positions = left.beacon_positions.clone();
        let left_set: HashSet<Position> = HashSet::from_iter(left_positions.0);

        let right = &scanners[right_idx.clone()];
        let right_positions = right.beacon_positions.clone();
        let right_set: HashSet<Position> = HashSet::from_iter(right_positions.0);
        
        let left_pairs: Vec<_> = left_set.iter().combinations(2).collect();
        let right_pairs: Vec<_> = right_set.iter().combinations(2).collect();

        let mut matches: Vec<((&Position, &Position), (&Position, &Position))> = vec![];

        for left_pair in &left_pairs {
            for right_pair in &right_pairs {
                let left_d = left_pair[0].distance(left_pair[1]);
                let right_d = right_pair[0].distance(right_pair[1]);
                if left_d.squared_euclidean == right_d.squared_euclidean {
                    matches.push(((left_pair[0], left_pair[1]), (right_pair[0], right_pair[1])));
                    break;
                }
            }
        }
        let first = matches[0];
        let left0: &Position = first.0.0;
        let left1: &Position = first.0.1;
        let right0: &Position = first.1.0;
        let right1: &Position = first.1.1;

        let left_1_in_0 = left1.in_basis(left0);

        let right_0_basis = Translation::in_basis(right0);
        let right_1_basis = Translation::in_basis(right1);
        let right_1_in_0 = right_0_basis.transform(right1);
        let right_0_in_1 = right_1_basis.transform(right0);

        for rot in &rotations {
            if left_1_in_0 == rot.transform(&right_1_in_0) {
                // positions match but they aren't in either scanner's basis
                let right_1_in_scanner_rotation = rot.transform(right1);
                println!("[{} -> {}] - {:?} == {:?}", left_idx, right_idx, left1, right_1_in_scanner_rotation);

                let matching = (rot.clone(), left1.difference(&right_1_in_scanner_rotation));
                // println!("[{} -> {}] - via {:?} then {:?}", left_idx, right_idx, matching.0, matching.1);
                scanner_transforms.push(matching);
            } else if left_1_in_0 == rot.transform(&right_0_in_1) {
                // positions match but they aren't in either scanner's basis
                let right_0_in_scanner_rotation = rot.transform(right0);
                println!("[{} -> {}] - {:?} == {:?}", left_idx, right_idx, left1, right_0_in_scanner_rotation);

                let matching = (rot.clone(), left1.difference(&right_0_in_scanner_rotation));
                // println!("[{} -> {}] - via {:?} then {:?}", left_idx, right_idx, matching.0, matching.1);
                scanner_transforms.push(matching);
            }
        }
    }

    for it in &scanner_transforms {
        println!("transforms: {:?}", it);
    }

    
    let mut intersections: HashMap<(usize, usize), HashSet<Position>> = HashMap::new();

    loop {
        if scanner_transforms.is_empty() {
            break;
        }

        let (rot, trans) = scanner_transforms.pop().unwrap();
        let (left, right) = scanner_pairs.pop().unwrap();

        let left_positions: HashSet<Position> = HashSet::from_iter(scanners[left].beacon_positions.0.clone());
        let right_positions: HashSet<Position> = HashSet::from_iter(scanners[right].beacon_positions.apply(&rot).apply(&trans).0);

        let i: HashSet<_> = left_positions.intersection(&right_positions).map(|p|p.clone()).collect();
        intersections.insert((left, right), i);
    }

    let zero = intersections.get(&(0, 1)).unwrap();
    for p in zero {
        println!("{},{},{}", p.x, p.y, p.z);
    }
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

fn nCr(n: i64, r: i64) -> i64 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn factorial(n: i64) -> i64 {
    match n  {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n
    }
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

    fn refect_basis(&self, other: &Self) -> Self {
        self.in_basis(&other.reflect(Axis::X).reflect(Axis::Y).reflect(Axis::Z))
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
    assert_eq!(transformed.refect_basis(&a), b);
    assert_eq!(b.in_basis(&a).refect_basis(&a), b);
}

#[test]
fn test_relative_positions() {
    let a = Position { x: 0, y: 0, z: 0 };
    let b = Position { x: 10, y: 10, z: 10 };
    let c_in_basis_b = Position { x: 5, y: 5, z: 5 };

    let c_in_basis_a = c_in_basis_b.refect_basis(&b);
    assert_eq!(c_in_basis_a, Position { x: 15, y: 15, z: 15 });
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

    let result = find_pairwise_rotation((
        &scanner0_positions[0],
        &scanner0_positions[1],
    ), (
        &scanner1_positions[0],
        &scanner1_positions[1],
    ));
    assert_eq!(rotation, result);
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
    let scanner_1: Position = (68,-1246,-43).into();

    let scanner_0_distance = scanner_0_points[0].distance(&scanner_0_points[1]);
    let scanner_1_distance = scanner_1_points[0].distance(&scanner_1_points[1]);

    assert_eq!(scanner_0_distance.squared_euclidean, scanner_1_distance.squared_euclidean);

    let rot = find_pairwise_rotation(
        (&scanner_0_points[0], &scanner_0_points[1]), 
        (&scanner_1_points[0], &scanner_1_points[1])
    );

    println!("rot: {:?}", rot);

    let rotated = [
        rot.transform(&scanner_1_points[0]),
        rot.transform(&scanner_1_points[1]),
    ];
    
    println!("rotated: {:?}", rotated);
}

fn find_pairwise_rotation(left: (&Position, &Position), right: (&Position, &Position)) -> Rotation {
    let s0_p1_in_0 = left.1.in_basis(left.0);

    let s1_p1_in_0 = right.1.in_basis(right.0);
    let s1_p0_in_1 = right.0.in_basis(right.1);

    println!("**");
    println!("{:?}", s0_p1_in_0);
    println!("{:?}", s1_p1_in_0);
    println!("{:?}", s1_p0_in_1);
    println!("**");

    let mut results: Vec<(Rotation, &Position)> = vec![];
    for rot in all_rotations() {
        if rot.transform(&s1_p1_in_0) == s0_p1_in_0 {
            results.push((rot, right.1));
        } 
        // else if rot.transform(&s1_p0_in_1) == s0_p1_in_0 {
        //     results.push((rot, right.0))
        // }
    }
    for it in &results {
        println!("{:?}", it);
    }
    results.pop().unwrap().0
}