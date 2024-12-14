use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{cartesian::{Plane, Point, Transform}, input, timing};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let robots = txt.lines().map(|l| l.parse::<Robot>().unwrap());
    let b = Bathroom::new(101, 103);
    let sim = robots.map(|robot| robot.simulate(100, &b)).collect_vec();
    b.safety_factor(&sim)
}

fn part2(txt: &str) -> i64 {
    let robots = txt.lines().map(|l| l.parse::<Robot>().unwrap()).collect_vec();
        let b = Bathroom::new(101, 103);

        let centre: HashSet<Point> = (25..75).flat_map(|i| vec![
            (25 + 25, 103 - i).into(),
            (25 + 26, 103 - i).into(),
            (25 + 27, 103 - i).into(),
            (25 + 28, 103 - i).into(),
            (25 + 29, 103 - i).into(),
            (25 + 30, 103 - i).into(),
            (25 + 31, 103 - i).into(),
            (25 + 32, 103 - i).into(),
            (25 + 33, 103 - i).into(),
            (25 + 34, 103 - i).into(),
            (25 + 35, 103 - i).into(),
            (25 + 36, 103 - i).into(),
            (25 + 37, 103 - i).into(),
            (25 + 38, 103 - i).into(),
            (25 + 39, 103 - i).into(),
            (25 + 40, 103 - i).into(),
            (25 + 41, 103 - i).into(),
            (25 + 42, 103 - i).into(),
            (25 + 43, 103 - i).into(),
            (25 + 44, 103 - i).into(),
            (25 + 45, 103 - i).into(),
            (25 + 46, 103 - i).into(),
            (25 + 47, 103 - i).into(),
            (25 + 48, 103 - i).into(),
            (25 + 49, 103 - i).into(),
            (25 + 50, 103 - i).into(),
        ]).collect();

        let mut most_intersections = 0;
        let mut seconds = 0;

        for i in 1..100000 {
            let sim = robots.iter().map(|robot| robot.simulate(i, &b)).collect_vec();
            let placed = b.place(&sim);
            let keys: HashSet<Point> = placed.keys().cloned().collect();
            let intersection_count = keys.intersection(&centre).count();
            if intersection_count > most_intersections {
                most_intersections = intersection_count;
                seconds = i;
            }
        }

        println!("{} at {}s", most_intersections, seconds);
        print_bathroom(&b, &robots.iter().map(|robot| robot.simulate(seconds, &b)).collect());
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    pos: Point,
    velocity: Transform,
}

struct Bathroom(Plane);

impl Bathroom {
    fn new(width: i64, height: i64) -> Bathroom {
        Bathroom(Plane { top_left: (0, 0).into(), bottom_right: (width - 1, height - 1).into() })
    }

    fn place(&self, robots: &Vec<Robot>) -> HashMap<Point, HashSet<Robot>> {
        let mut robot_positions: HashMap<Point, HashSet<Robot>> = HashMap::new();
        for robot in robots {
            if let Some(existing) = robot_positions.get_mut(&robot.pos) {
                existing.insert(robot.clone());
            } else {
                robot_positions.insert(robot.pos.clone(), HashSet::from_iter(vec![robot.clone()]));
            }
        }
        robot_positions
    }

    fn safety_factor(&self, robots: &Vec<Robot>) -> i64 {
        let robot_positions = self.place(robots);

        let ignore_y = self.0.height() / 2;
        let ignore_x = self.0.width() / 2;

        let mut quadrants = [[0_i64; 2]; 2];


        for y in 0..self.0.height() {
            for x in 0..self.0.width() {                
                if y != ignore_y && x != ignore_x {

                    let x_quandrant = (x / ((self.0.width() + 1) / 2)) as usize;
                    let y_quandrant = (y / ((self.0.height() + 1) / 2)) as usize;
                    // let quadrant = (x_quandrant + y_quandrant) as usize;

                    // println!("x {} -> Qx {} w {}", x, x_quandrant, self.0.width());
                    // println!("y {} -> Qy {} h {}", y, y_quandrant, self.0.height());
                    // println!("{},{} = Q{}\n", x, y, quadrant);

                    let pos: Point = (x, y).into();
                    if let Some(robots_here) = robot_positions.get(&pos) {
                        quadrants[x_quandrant][y_quandrant] += robots_here.len() as i64;
                    }
                }
            }
        }

        // println!("Quads: {:?}", quadrants);

        quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
    }
}

impl Robot {
    fn simulate(&self, seconds: i64, b: &Bathroom) -> Robot {
        let transform: Transform = self.velocity.clone() * seconds;
        let new_pos = self.pos.transform(&transform);

        let scaled_pos = Point {
            x: new_pos.x.rem_euclid(b.0.width()),
            y: new_pos.y.rem_euclid(b.0.height()),
        };

        Robot { pos: scaled_pos, velocity: self.velocity.clone() }
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" ").unwrap();
        let (_, pair) = left.split_once("=").unwrap();
        let (x, y) = pair.split_once(",").unwrap();
        let (_, pair) = right.split_once("=").unwrap();
        let (dx, dy) = pair.split_once(",").unwrap();

        Ok(Robot { 
            pos: (x.parse().unwrap(), y.parse().unwrap()).into(), 
            velocity: (dx.parse().unwrap(), dy.parse().unwrap()).into() 
        })
    }
}

#[allow(dead_code)]
fn print_bathroom(b: &Bathroom, r: &Vec<Robot>) {
    let placed = b.place(r);
    
    for y in 0..b.0.height() {
        for x in 0..b.0.width() {
            let p = Point::new(x, y);
            if let Some(here) = placed.get(&p) {
                print!("{}", here.len());
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_robot_movement() {
        let robots: Vec<Robot> = vec!["p=2,4 v=2,-3".parse().unwrap()];        
        let b = Bathroom::new(11, 7);
        let sim_five = robots.iter().map(|robot| robot.simulate(5, &b)).collect_vec();
        assert_eq!(Point::new(1, 3), sim_five[0].pos);
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        let robots = test_input.lines().map(|l| l.parse::<Robot>().unwrap()).collect_vec();
        let b = Bathroom::new(11, 7);
        
        let sim_part1 = robots.iter().map(|robot| robot.simulate(100, &b)).collect_vec();
        print_bathroom(&b, &sim_part1);
        assert_eq!(12, b.safety_factor(&sim_part1));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        let robots = test_input.lines().map(|l| l.parse::<Robot>().unwrap()).collect_vec();
        let b = Bathroom::new(101, 103);

        let trunk: HashSet<Point> = (1..50).map(|i| (50, 103 - i).into()).collect();

        for i in 1..100000 {
            let sim = robots.iter().map(|robot| robot.simulate(i, &b)).collect_vec();
            let placed = b.place(&sim);
            let keys: HashSet<Point> = placed.keys().cloned().collect();
            if (keys.intersection(&trunk).count()) > 10 {
                println!("Seconds: {}", i);
                print_bathroom(&b, &sim);
            }            
            println!("\n\n");
        }

        assert_eq!(0, part2(test_input));
    }
}
