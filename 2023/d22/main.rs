use std::{time::Instant, str::FromStr, ops::{Range, RangeInclusive, Mul}, cmp::Ordering, vec};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let snap = {
        let mut s: Snapshot = txt.parse().unwrap();
        s.fall();
        Snapshot(s.into_iter().sorted_by(|left, right| left.min_z().cmp(&right.min_z())).collect())
    };

    let (_, z_max) = snap.z_bounds();

    let mut z_max_index: Vec<Vec<usize>> = vec![vec![]; z_max as usize + 1];
    let mut z_min_index: Vec<Vec<usize>> = vec![vec![]; z_max as usize + 1];

    for (i, b) in snap.0.iter().enumerate() {
        let (z_min, z_max) = (b.min_z(), b.max_z());
        z_max_index[z_max as usize].push(i);
        z_min_index[z_min as usize].push(i);
    }

    let mut supports_me: Vec<Vec<usize>> = vec![vec![]; snap.0.len()];
    let mut i_support: Vec<Vec<usize>> = vec![vec![]; snap.0.len()];

    for (i, b) in snap.0.iter().enumerate() {
        let (z_min, _z_max) = (b.min_z(), b.max_z());
        let moved_down = b.transform_into(Transform3D::down());
        let below_bricks_z_index = z_min - 1;
        
        if below_bricks_z_index > 0 {
            let below_bricks = &z_max_index[below_bricks_z_index as usize];
            for below_brick_idx in below_bricks {
                let below_brick = &snap.0[*below_brick_idx];
                if moved_down.overlaps(below_brick) {
                    supports_me[i].push(*below_brick_idx);
                    i_support[*below_brick_idx].push(i);
                }
            }
        };

    }

    let mut count = 0;
    for (i, b_idx) in i_support.iter()
        .enumerate()
        .filter(|(_i, supported)| supported.len() > 0) {

        // if all the blocks I support have at least 1 other support, then we can be removed
        if b_idx.iter().all(|test| supports_me[*test].len() > 1) {
            count += 1;
        }

    }

    // for (i, b) in snap.0.iter().enumerate() {
    //     println!("{} = {:?}", i, b);
    // }

    // for (i, i_supports) in i_support.iter().enumerate() {
    //     println!("{} supports {:?}", i, i_supports);
    // }

    let supports_none = i_support.iter().filter(|support_count| support_count.len() == 0).count();

    for empty in i_support.iter().enumerate().filter(|(_, support_count)| support_count.len() == 0) {
        println!("supports none: {:?}", snap.0[empty.0]);
    }

    // for b in snap.z_ordered().enumerate() {
        // println!("{:?}", b);
    // }

    count + supports_none
}

fn part2(txt: &str) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3D(i64, i64, i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Transform3D(i64, i64, i64);

impl Point3D {
    fn x(&self) -> i64 {
        self.0
    }

    fn y(&self) -> i64 {
        self.1
    }

    fn z(&self) -> i64 {
        self.2
    }

    fn transform_mut(&mut self, t: Transform3D) {
        self.0 = self.0 + t.0;
        self.1 = self.1 + t.1;
        self.2 = self.2 + t.2;
    }

    fn transform_into(&self, t: Transform3D) -> Self {
        let new_x = self.0 + t.0;
        let new_y = self.1 + t.1;
        let new_z = self.2 + t.2;

        Self(new_x, new_y, new_z)
    }
}

impl Transform3D {
    fn down() -> Self {
        Self(0, 0, -1)
    }

    fn up() -> Self {
        Self(0, 0, 1)
    }
}

impl Mul<i64> for Transform3D {
    type Output = Transform3D;

    fn mul(self, rhs: i64) -> Self::Output {
        Transform3D(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum Brick {
    X(RangeInclusive<i64>, i64, i64),
    Y(i64, RangeInclusive<i64>, i64),
    Z(i64, i64, RangeInclusive<i64>),
    Point(Point3D),
}

impl Brick {
    fn min_z(&self) -> i64 {
        match self {
            Brick::X(_, _, z) => *z,
            Brick::Y(_, _, z) => *z,
            Brick::Z(_, _, z) => *z.start(),
            Brick::Point(Point3D(_, _, z)) => *z,
        }
    }

    fn max_z(&self) -> i64 {
        match self {
            Brick::X(_, _, z) => *z,
            Brick::Y(_, _, z) => *z,
            Brick::Z(_, _, z) => *z.end(),
            Brick::Point(Point3D(_, _, z)) => *z,
        }
    }

    // fn contains(&self, point: Point3D) -> bool {
    //     match self {
    //         Brick::X(x, y, z) => x.contains(&point.x()) && *y == point.y() && *z == point.z(),
    //         Brick::Y(x, y, z) => *x == point.x() && y.contains(&point.y()) && *z == point.z(),
    //         Brick::Z(x, y, z) => *x == point.x() && *y == point.y() && z.contains(&point.z()),
    //         Brick::Point(p) => *p == point,
    //     }
    // }

    fn overlaps(&self, other: &Self) -> bool {
        match self {
            Brick::X(x, y, z) => match other {
                Brick::X(other_x, other_y, other_z) => y == other_y && z == other_z && overlaps(x, other_x),
                Brick::Y(other_x, other_y, other_z) => x.contains(other_x) && other_y.contains(y) && z == other_z,
                Brick::Z(other_x, other_y, other_z) => x.contains(other_x) && y == other_y && other_z.contains(z),
                Brick::Point(Point3D(other_x, other_y, other_z)) => x.contains(other_x) && y == other_y && z == other_z,
            },
            Brick::Y(x, y, z) => match other {
                Brick::X(other_x, other_y, other_z) => other_x.contains(x) && y.contains(other_y) && z == other_z,
                Brick::Y(other_x, other_y, other_z) => x == other_x && overlaps(y, other_y) && z == other_z,
                Brick::Z(other_x, other_y, other_z) => x == other_x && y.contains(other_y) && other_z.contains(z),
                Brick::Point(Point3D(other_x, other_y, other_z)) => x == other_x && y.contains(other_y) && z == other_z,
            },
            Brick::Z(x, y, z) => match other {
                Brick::X(other_x, other_y, other_z) => other_x.contains(x) && y == other_y && z.contains(other_z),
                Brick::Y(other_x, other_y, other_z) => x == other_x && other_y.contains(y) && z.contains(other_z),
                Brick::Z(other_x, other_y, other_z) => x == other_x && y == other_y && overlaps(z, other_z),
                Brick::Point(Point3D(other_x, other_y, other_z)) => x == other_x && y == other_y && z.contains(other_z),
            },
            Brick::Point(p) => match other {
                Brick::X(other_x, other_y, other_z) => other_x.contains(&p.x()) && p.y() == *other_y && p.z() == *other_z,
                Brick::Y(other_x, other_y, other_z) => p.x() == *other_x && other_y.contains(&p.y()) && p.z() == *other_z,
                Brick::Z(other_x, other_y, other_z) => p.x() == *other_x && p.y() == *other_y && other_z.contains(&p.z()),
                Brick::Point(other_p) => p == other_p,
            },
        }
    }

    fn transform_into(&self, t: Transform3D) -> Brick {
        match self {
            Brick::X(x, y, z) => Brick::X((x.start() + t.0)..=(x.end() + t.0), y + t.1, z + t.2),
            Brick::Y(x, y, z) => Brick::Y(x + t.0, (y.start() + t.1)..=(y.end() + t.1), z + t.2),
            Brick::Z(x, y, z) => Brick::Z(x + t.0, y + t.1, (z.start() + t.2)..=(z.end() + t.2)),
            Brick::Point(p) => Brick::Point(p.transform_into(t)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Snapshot(Vec<Brick>);

impl IntoIterator for Snapshot {
    type Item = Brick;

    type IntoIter = std::vec::IntoIter<Brick>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Snapshot {        

    fn z_bounds(&self) -> (i64, i64) {
        self.0.iter().fold((i64::MAX, i64::MIN), |(min, max), brick| {
            match brick {
                Brick::X(_, _, z) => (min.min(*z), max.max(*z)),
                Brick::Y(_, _, z) => (min.min(*z), max.max(*z)),
                Brick::Z(_, _, z) => (min.min(*z.start()), max.max(*z.end())),
                Brick::Point(Point3D(_, _, z)) => (min.min(*z), max.max(*z)),
            }
        })
    }

    fn z_ordered_mut(&mut self) -> impl Iterator<Item = &mut Brick> + DoubleEndedIterator<Item = &mut Brick> {
        self.0
            .iter_mut()        
            .sorted_by(|left, right| {
                let left_z = left.min_z();
                let right_z = right.min_z();
                left_z.cmp(&right_z)
            })
    }

    fn z_ordered(&self) -> impl Iterator<Item = &Brick> + DoubleEndedIterator<Item = &Brick> {
        self.0
            .iter()        
            .sorted_by(|left, right| {
                let left_z = left.min_z();
                let right_z = right.min_z();
                left_z.cmp(&right_z)
            })
    }

    fn fall(&mut self) {
        let (min_z, max_z) = self.z_bounds();
        let z_size = (max_z - min_z) as usize;


        let mut z_ordered: Vec<Vec<&Brick>> = vec![vec![]; z_size + 1];

        // iterate from bottom (min z) to top        
        for b in self.z_ordered_mut() {
            
            let z = b.min_z();
            let mut offset = (z - min_z) as usize;

            loop {
                if offset < 1 {
                    break;
                }

                let down_one = b.transform_into(Transform3D::down());                
                let below_z = offset - 1;
                if !z_ordered[below_z].iter().any(|below| below.overlaps(&down_one)) {                    
                    // there are non below that overlap, so move down
                    *b = down_one;
                    offset -= 1;
                } else {
                    break;
                }
            }
            let current = &mut z_ordered[offset];
            current.push(b);
        }
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split("~").collect_tuple().unwrap();
        let left_point: Point3D = left.parse().unwrap();
        let right_point: Point3D = right.parse().unwrap();

        let same_x = left_point.0 == right_point.0;
        let same_y = left_point.1 == right_point.1;
        let same_z = left_point.2 == right_point.2;

        let b = match (same_x, same_y, same_z) {            
            (false, false, false) => panic!("all points are different"),
            (true, false, false) => panic!("y and z different"),
            (false, true, false) => panic!("x and z different"),
            (false, false, true) => panic!("x and y different"),

            (true, true, false) => Brick::Z(left_point.x(), right_point.y(), left_point.2.min(right_point.2)..=left_point.2.max(right_point.2)),
            (true, false, true) => Brick::Y(left_point.x(), left_point.y().min(right_point.y())..=left_point.y().max(right_point.y()), left_point.z()),            
            (false, true, true) => Brick::X(left_point.x().min(right_point.x())..=left_point.x().max(right_point.x()), right_point.y(), left_point.z()),
            (true, true, true) => Brick::Point(left_point),
        };

        Ok(b)
    }
}

impl FromStr for Point3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) =  s.split(",").collect_tuple().unwrap();
        Ok(Point3D(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
    }
}

impl FromStr for Snapshot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snapshot(s.lines().map(|l| l.parse().unwrap()).collect()))
    }
}

fn overlaps(left: &RangeInclusive<i64>, right: &RangeInclusive<i64>) -> bool {
    right.contains(left.start()) || right.contains(left.end()) || left.contains(right.start()) || left.contains(right.end())
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(5, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let txt = include_str!("input.test.txt");
        let snap: Snapshot = txt.parse().unwrap();
        assert_eq!(7, snap.0.len());
        assert_eq!(Brick::Y(1,0..=2,1), snap.0[0]);
    }

    #[test]
    fn test_sort_by_z() {
        let txt = include_str!("input.test.txt");
        let mut snap: Snapshot = txt.parse().unwrap();
        let sorted: Vec<_> = snap.z_ordered_mut().collect();

        for b in sorted {
            println!("{:?}", b);
        }        
    }

    #[test]
    fn test_transform_into() {
        let p = Point3D(0, 0, 10);

        assert_eq!(Point3D(0, 0, 5), p.transform_into(Transform3D::down() * 5))
    }

    #[test]
    fn test_transform() {
        let mut p = Point3D(0, 0, 10);
        p.transform_mut(Transform3D::down() * 5);

        assert_eq!(Point3D(0, 0, 5), p);
    }

    #[test]
    fn test_overlaps() {
        let b1 = Brick::Y(0, 0..=10, 0);
        let b2 = Brick::Y(0, 5..=6, 0);
        let b3 = Brick::Y(0, 5..=6, 1);

        assert_eq!(true, b1.overlaps(&b2));
        assert_eq!(false, b1.overlaps(&b3));
        assert_eq!(false, b2.overlaps(&b3));
    }

    #[test]
    fn test_fall() {
        let mut snap = Snapshot(vec![
            Brick::Y(0, 11..=12, 5),
            Brick::Y(0, 0..=10, 5),
            Brick::Y(0, 0..=10, 1),
        ]);

        snap.fall();

        let expected = Snapshot(vec![
            Brick::Y(0, 11..=12, 1),
            Brick::Y(0, 0..=10, 2),
            Brick::Y(0, 0..=10, 1),
        ]);

        assert_eq!(expected, snap);
    }

    #[test]
    fn test_pt1_example_after_fall() {
        let txt = include_str!("input.test.txt");
        let mut snap: Snapshot = txt.parse().unwrap();

        let sorted: Vec<_> = snap.z_ordered_mut()
            .rev()
            .collect();
        for b in sorted {
            println!("{:?}", b);
        }

        println!();
        println!();

        snap.fall();

        let sorted: Vec<_> = snap.z_ordered_mut()
            .rev()
            .collect();
        for b in sorted {
            println!("{:?}", b);
        }

        assert_eq!(1, snap.0[0].min_z()); // a

        assert_eq!(2, snap.0[1].min_z()); // b
        assert_eq!(2, snap.0[2].min_z()); // c

        assert_eq!(3, snap.0[3].min_z()); // d
        assert_eq!(3, snap.0[4].min_z()); // e

        assert_eq!(4, snap.0[5].min_z()); // f

        assert_eq!(5, snap.0[6].min_z()); // g
    }

    #[test]
    fn test_pt1_fall_twice() {
        let txt = include_str!("input.txt");        
        let mut snap: Snapshot = txt.parse().unwrap();
        snap.fall();
        let before_twice = snap.clone();
        snap.fall();

        assert_eq!(before_twice, snap);

        for b in snap.0.iter().filter(|b| b.max_z() == 6 || b.min_z() == 6) {
            println!("below {:?}", b);
        }

        let a = Brick::Y(9, 4..=7, 6);
        let b = Brick::Y(9, 5..=6, 7);
        let b_down = b.transform_into(Transform3D::down());

        assert_eq!(true, b_down.overlaps(&a), "b_down: {:?}", b_down);

        for b in snap.0.iter().filter(|b| b.min_z() != 1) {
            let down = b.transform_into(Transform3D::down());
            let collides = snap.0.iter().any(|test| down.overlaps(test));
            assert_eq!(true, collides, "{:?} doesn't collide with any when moved down", b);
        }

        let three = &snap.0[3];
        println!("three = {:?}", three);
    }
}
