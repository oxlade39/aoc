use std::{collections::HashSet, str::FromStr, time::Instant};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} μs", start.elapsed().as_micros())
}

fn part1() {
    let input = include_str!("input.txt");
    let target_area = input.parse::<Box>().unwrap();

    let mut max_y = 0;
    let mut end_y = 0;
    let mut max_n = 0;

    let mut trial_y_v = 0;
    loop {
        let mut max_y_in_run = 0;
        for n in 0..1000000000 {
            let y_pos = calc_y(n, trial_y_v);
            max_y_in_run = i64::max(max_y_in_run, y_pos);
            max_n = i64::max(max_n, n);
            if y_pos < target_area.bottom() {
                break;
            }
            end_y = y_pos;
        }
        trial_y_v += 1;

        if end_y == 0 {
            break;
        }
        max_y = i64::max(max_y, max_y_in_run);
    }
    println!("part1: {}", max_y);
}

fn part2() {
    let input = include_str!("input.txt");
    let target_area = input.parse::<Box>().unwrap();
    let mut matches: HashSet<(i64, i64)> = HashSet::new();

    for n in 0..1000 {
        for x in 0..100 {
            for y in target_area.bottom()..1000 {
                let x_pos = calc_x(n, x);
                let y_pos = calc_y(n, y);
                let p = Point { x: x_pos, y: y_pos };
                let velocity = (x, y);
                if target_area.contains(&p) {
                    matches.insert(velocity);
                }
            }
        }
    }

    println!("part2: {}", matches.len());
}

fn calc_y(n: i64, v: i64) -> i64 {
    (n * v) - (((n - 1) * n) / 2)
}

fn calc_x(n: i64, v: i64) -> i64 {
    let modified_n = i64::min(n, v);
    let inc = modified_n * v;
    let sub = ((modified_n - 1) * modified_n) / 2;
    let result = inc - sub;
    result
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Box {
    top_left: Point,
    bottom_right: Point,
}

impl Box {
    fn new(a: Point, b: Point) -> Self {
        let top_left = Point {
            x: i64::min(a.x, b.x),
            y: i64::max(a.y, b.y),
        };
        let bottom_right = Point {
            x: i64::max(a.x, b.x),
            y: i64::min(a.y, b.y),
        };
        Self {
            top_left,
            bottom_right,
        }
    }

    fn left(&self) -> i64 {
        self.top_left.x
    }

    fn right(&self) -> i64 {
        self.bottom_right.x
    }

    fn top(&self) -> i64 {
        self.top_left.y
    }

    fn bottom(&self) -> i64 {
        self.bottom_right.y
    }

    fn contains(&self, p: &Point) -> bool {
        let within_x = p.x >= self.left() && p.x <= self.right();
        let within_y = p.y >= self.bottom() && p.y <= self.top();
        within_x && within_y
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseErr;

impl FromStr for Box {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s["target area: ".len()..].split(", ").collect();

        let left: Vec<_> = parts[0].split("=").collect();
        let xs: Vec<_> = left[1].split("..").collect();
        let x1: i64 = xs[0].parse().unwrap();
        let x2: i64 = xs[1].parse().unwrap();

        let right: Vec<_> = parts[1].split("=").collect();
        let ys: Vec<_> = right[1].split("..").collect();
        let y1: i64 = ys[0].parse().unwrap();
        let y2: i64 = ys[1].parse().unwrap();

        Ok(Box::new(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }))
    }
}

#[test]
fn test_calc_y() {
    assert_eq!(2, calc_y(1, 2));
    assert_eq!(3, calc_y(2, 2));
    assert_eq!(3, calc_y(3, 2));
    assert_eq!(2, calc_y(4, 2));
    assert_eq!(0, calc_y(5, 2));
    assert_eq!(-3, calc_y(6, 2));
}

#[test]
fn test_calc_x() {
    assert_eq!(7, calc_x(1, 7));
    assert_eq!(13, calc_x(2, 7));
    assert_eq!(18, calc_x(3, 7));
    assert_eq!(22, calc_x(4, 7));
    assert_eq!(25, calc_x(5, 7));
    assert_eq!(27, calc_x(6, 7));
    assert_eq!(28, calc_x(7, 7));
    assert_eq!(28, calc_x(8, 7));
    assert_eq!(28, calc_x(9, 7));
    assert_eq!(28, calc_x(10, 7));
    assert_eq!(28, calc_x(1000, 7));

    // max x velocity is always a triangular num
    assert_eq!(1, calc_x(1000, 1));
    assert_eq!(3, calc_x(1000, 2));
    assert_eq!(6, calc_x(1000, 3));
    assert_eq!(10, calc_x(1000, 4));
    assert_eq!(15, calc_x(1000, 5));
    assert_eq!(21, calc_x(1000, 6));
    assert_eq!(28, calc_x(1000, 7));
    assert_eq!(36, calc_x(1000, 8));
    assert_eq!(45, calc_x(1000, 9));
    assert_eq!(55, calc_x(1000, 10));
    assert_eq!(66, calc_x(1000, 11));
    assert_eq!(78, calc_x(1000, 12));
    assert_eq!(91, calc_x(1000, 13));
    assert_eq!(105, calc_x(1000, 14));
}

#[test]
fn test_box_contains() {
    let b = Box::new(Point { x: 20, y: -10 }, Point { x: 30, y: -5 });

    assert_eq!(true, b.contains(&Point { x: 20, y: -5 }));
    assert_eq!(true, b.contains(&Point { x: 30, y: -5 }));

    assert_eq!(false, b.contains(&Point { x: 19, y: -5 }));
    assert_eq!(false, b.contains(&Point { x: 31, y: -5 }));

    assert_eq!(true, b.contains(&Point { x: 20, y: -5 }));
    assert_eq!(true, b.contains(&Point { x: 20, y: -10 }));

    assert_eq!(false, b.contains(&Point { x: 20, y: -4 }));
    assert_eq!(false, b.contains(&Point { x: 20, y: -11 }));
}

#[test]
fn test_parse_box() {
    let input = "target area: x=20..30, y=-10..-5";
    let b = input.parse::<Box>();

    assert_eq!(
        Ok(Box {
            top_left: Point { x: 20, y: -5 },
            bottom_right: Point { x: 30, y: -10 }
        }),
        b
    );
}
