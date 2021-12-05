use std::collections::HashSet;



fn main() {
    let input = include_str!("input.txt");

    let mut vent_points: HashSet<Point> = HashSet::new();
    let mut vent_collision: HashSet<Point> = HashSet::new();

    for vent_point in input.lines()
        .flat_map(|l| {
            let parts: Vec<&str> = l.split(" -> ").collect();
            let left_point: Vec<&str> = parts[0].split(",").collect();
            let right_point: Vec<&str> = parts[1].split(",").collect();

            let left_x = left_point[0].parse().unwrap();
            let left_y = left_point[1].parse().unwrap();
            let right_x = right_point[0].parse().unwrap();
            let right_y = right_point[1].parse().unwrap();
            LineSegment(Point{ x: left_x, y: left_y}, Point{ x: right_x, y: right_y })
        }) {
        if !vent_points.insert(vent_point) {
            vent_collision.insert(vent_point);
        }
    }

    println!("vent_collisions: {:?}", vent_collision.len());
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct LineSegment(Point, Point);

impl IntoIterator for LineSegment {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        if self.0.x == self.1.x {
            let mn = i32::min(self.0.y, self.1.y);
            let mx = i32::max(self.0.y, self.1.y);
            let v: Vec<_> = (mn..=mx)
                .map(|i| Point{ x: self.0.x, y: i })
                .collect();
            v.into_iter()
        } else if self.0.y == self.1.y {
            let mn = i32::min(self.0.x, self.1.x);
            let mx = i32::max(self.0.x, self.1.x);
            let v: Vec<_> = (mn..=mx)
                .map(|i| Point{ x: i, y: self.0.y })
                .collect();
            v.into_iter()
        } else {
            let x_direction = if self.0.x > self.1.x {
                -1
            } else {
                1
            };
            let y_direction = if self.0.y > self.1.y {
                -1
            } else {
                1
            };

            let diag_len = i32::abs(self.0.x - self.1.x);
            let v: Vec<_> = (0..=diag_len)
                .map(|n| Point{ x: self.0.x + n * x_direction, y: self.0.y + n * y_direction})
                .collect();
            v.into_iter()
        }
    }
}


#[test]
fn line_segment_along_y_into_iter() {
    let ls = LineSegment(Point{ x: 5, y: 1 }, Point{ x: 5, y: 3 });
    let v: Vec<_> = ls.into_iter().collect();

    assert_eq!(vec![Point{ x: 5, y: 1}, Point{ x: 5, y: 2}, Point{ x: 5, y: 3}], v);
}

#[test]
fn line_segment_along_x_into_iter() {
    let ls = LineSegment(Point{ x: 1, y: 1 }, Point{ x: 3, y: 1 });
    let v: Vec<_> = ls.into_iter().collect();

    assert_eq!(vec![Point{ x: 1, y: 1}, Point{ x: 2, y: 1}, Point{ x: 3, y: 1}], v);
}


#[test]
fn line_segment_along_diagonal_simple_into_iter() {
    // 1,1 -> 3,3
    let ls = LineSegment(Point{ x: 1, y: 1 }, Point{ x: 3, y: 3 });
    let v: Vec<_> = ls.into_iter().collect();

    assert_eq!(vec![Point{ x: 1, y: 1}, Point{ x: 2, y: 2}, Point{ x: 3, y: 3}], v);
}

#[test]
fn line_segment_along_diagonal_complex_into_iter() {
    let ls = LineSegment(Point{ x: 9, y: 7 }, Point{ x: 7, y: 9 });
    let v: Vec<_> = ls.into_iter().collect();

    assert_eq!(vec![Point{ x: 9, y: 7}, Point{ x: 8, y: 8}, Point{ x: 7, y: 9}], v);
}