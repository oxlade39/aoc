use core::str;
use std::{i64, str::FromStr, time::Instant};

use aoclib::timing;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let tiles: Vec<Point> = txt.lines().map(|l| l.parse::<Point>().unwrap()).collect();
    tiles
        .iter()
        .combinations(2)
        .map(|comb| {
            let a = comb[0];
            let b = comb[1];
            a.to_rect(b).area()
        })
        .max()
        .expect("a max")
}

fn part2(txt: &str) -> i64 {
    let red_tiles: Vec<Point> = txt.lines().map(|l| l.parse::<Point>().unwrap()).collect();
    let sorted_by_x: HashMap<i64, usize> = red_tiles
        .iter()
        .map(|p| p.x)
        .unique()
        .sorted()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
    let sorted_by_y: HashMap<i64, usize> = red_tiles
        .iter()
        .map(|p| p.y)
        .unique()
        .sorted()
        .enumerate()
        .map(|(i, y)| (y, i))
        .collect();
    let compressed_red_tiles: Vec<_> = red_tiles
        .iter()
        .map(|Point { x, y }| Point {
            x: *sorted_by_x.get(x).unwrap() as i64,
            y: *sorted_by_y.get(y).unwrap() as i64,
        })
        .collect();
    
    let mut red_lines: Vec<Line> = compressed_red_tiles
        .iter()
        .tuple_windows()
        .map(|(p, next)| p.line_between(next))
        .collect();

    let first = compressed_red_tiles.first().unwrap();
    let last = compressed_red_tiles.last().unwrap();
    red_lines.push(first.line_between(last));

    let poly = Poly::new(red_lines.clone());

    compressed_red_tiles
        .iter()
        .enumerate()
        .combinations(2)
        .par_bridge()
        .filter_map(|comb| {
            let (i, a) = comb[0];
            let (j, b) = comb[1];
            let rect = a.to_rect(b);

            let top_left = Point {
                x: rect.bottom_left.x,
                y: rect.top_right.y,
            };
            let top_right = rect.top_right;
            let bottom_left = rect.bottom_left;
            let bottom_right = Point {
                x: top_right.x,
                y: bottom_left.y,
            };

            let top_edge = top_left.line_between(&top_right);
            let right_edge = top_right.line_between(&bottom_right);
            let bottom_edge = bottom_left.line_between(&bottom_right);
            let left_edge = bottom_left.line_between(&top_left);

            let ps = {
                let mut all_points = HashSet::new();
                all_points.extend(top_edge.points());
                all_points.extend(right_edge.points());
                all_points.extend(bottom_edge.points());
                all_points.extend(left_edge.points());
                all_points
            };

            if ps.iter().all(|&pt| poly.contains_point(pt)) {
                let orig_a = red_tiles[i];
                let orig_b = red_tiles[j];
                Some(orig_a.to_rect(&orig_b).area())
            } else {
                None
            }
        })
        .max()
        .expect("a max")
}

#[allow(dead_code)]
fn print_green(green: &HashSet<Point>) {
    let rows = 10;
    let cols = 14;
    print_points(green, rows, cols);
}

#[allow(dead_code)]
fn print_points(green: &HashSet<Point>, rows: i64, cols: i64) {
    for i in 0..rows {
        for j in 0..cols {
            let p = Point { x: j, y: i };
            if green.contains(&p) {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!("");
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rect {
    top_right: Point,
    bottom_left: Point,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    fn to_rect(&self, p: &Point) -> Rect {
        Rect {
            top_right: Point {
                x: self.x.max(p.x),
                y: self.y.min(p.y),
            },
            bottom_left: Point {
                x: self.x.min(p.x),
                y: self.y.max(p.y),
            },
        }
    }

    fn line_between(&self, p: &Point) -> Line {
        Line {
            start: Point {
                x: self.x.min(p.x),
                y: self.y.min(p.y),
            },
            end: Point {
                x: self.x.max(p.x),
                y: self.y.max(p.y),
            },
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

impl Line {
    fn points(&self) -> HashSet<Point> {
        let mut all = HashSet::new();
        if self.start.x == self.end.x {
            // points along x
            for y in self.start.y.min(self.end.y)..=self.start.y.max(self.end.y) {
                all.insert(Point { x: self.start.x, y });
            }
        } else {
            // points along y
            for x in self.start.x.min(self.end.x)..=self.start.x.max(self.end.x) {
                all.insert(Point { x, y: self.end.y });
            }
        }
        all
    }
}

struct Poly {
    all_border_points: HashSet<Point>,
    edges: Vec<Line>,
}

impl Poly {
    fn new(lines: Vec<Line>) -> Self {
        let all_border_points: HashSet<_> = lines.iter().flat_map(|e| e.points()).collect();

        // print_points(&all_border_points, 1000, 1000);
        // panic!("oops");

        Self {
            all_border_points,
            edges: lines,
        }
    }

    fn contains_point(&self, p: Point) -> bool {
        let Point { x, y } = p;
        let mut inside = false;

        if self.all_border_points.contains(&p) {
            return true;
        }

        for edge in &self.edges {
            let Line {
                start: Point { x: x1, y: y1 },
                end: Point { x: x2, y: y2 },
            } = edge;
            if y1 == y2 {
                // ignore horizontals
                continue;
            }
            let y_min = *y1.min(y2);
            let y_max = *y1.max(y2);
            if !(y_min <= y && y < y_max) {
                continue;
            }

            // Compute x-intersection
            let x_intersect = *x1 + (y - *y1) * (*x2 - *x1) / (*y2 - *y1);

            if x_intersect > x {
                inside = !inside;
            }
        }

        inside
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
        let a = Point { x: 2, y: 5 };
        let b = Point { x: 9, y: 7 };
        let area = a.to_rect(&b).area();
        assert_eq!(24, area);

        let a = Point { x: 7, y: 1 };
        let b = Point { x: 11, y: 7 };
        let area = a.to_rect(&b).area();
        assert_eq!(35, area);

        let a = Point { x: 7, y: 3 };
        let b = Point { x: 2, y: 3 };
        let area = a.to_rect(&b).area();
        assert_eq!(6, area);
    }

    #[test]
    fn line_points() {
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 2 },
        };
        let points = line.points();
        let expected = HashSet::from_iter(vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
        ]);
        assert_eq!(expected, points);

        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 2, y: 0 },
        };
        let points = line.points();
        let expected = HashSet::from_iter(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ]);
        assert_eq!(expected, points);
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
        assert_eq!(24, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1568849600, part2(test_input));
    }
}
