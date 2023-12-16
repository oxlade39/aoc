use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
    time::Instant,
};

use aoclib::{
    cartesian::{Plane, Point, Transform},
    input::Grid,
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i64 {
    let d: Dish = txt.parse().unwrap();
    tilt(d, &Tilt::North).score()
}

const TILTS: [Tilt; 4] = [Tilt::North, Tilt::West, Tilt::South, Tilt::East];

fn part2(txt: &str) -> i64 {
    find_result_n(txt, 1000000000)
}

fn find_result_n(txt: &str, n: i32) -> i64 {
    let mut dish: Dish = txt.parse().unwrap();

    let mut store: HashMap<String, i32> = HashMap::new();
    let mut items: Vec<i64> = Vec::new();
    let mut cycle = (0, 0);

    for i in 1.. {
        dish = dish.cycle();
        items.push(dish.score());
        if let Some(old) = store.insert(format!("{dish}"), i) {
            cycle = (old, i);
            break;
        }
    }

    let target = n;
    let (cycle_start, cycle_end) = cycle;
    let cycle_start = cycle_start - 1;
    let cycle_end = cycle_end - 1;
    let cycle_length = cycle_end - cycle_start;
    // 1000000000 = cycle_start + cycle_length * X

    let offset = (target - cycle_start) % cycle_length;

    let result = items[((cycle_start - 1) as usize) + offset as usize];
    result
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Dish {
    round: HashSet<Point>,
    square: HashSet<Point>,
    plane: Plane,
}

impl Dish {
    fn score(&self) -> i64 {
        self.round.iter().map(|rock| rock.y + 1).sum()
    }

    fn cycle(self) -> Self {
        let mut d = self;
        for t in TILTS.iter() {
            d = tilt(d, t);
        }
        d
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tilt {
    East,
    West,
    North,
    South,
}

impl From<&Tilt> for Transform {
    fn from(value: &Tilt) -> Self {
        match *value {
            Tilt::East => (1, 0).into(),
            Tilt::West => (-1, 0).into(),
            Tilt::North => (0, 1).into(),
            Tilt::South => (0, -1).into(),
        }
    }
}

impl fmt::Display for Dish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = fmt::Result::Ok(());
        let width = self.plane.width();
        let height = self.plane.height();
        for row in 0..height {
            let y = height - row - 1;
            for x in 0..width {
                let p: Point = (x as i64, y as i64).into();
                if self.round.contains(&p) {
                    result = result.and_then(|_| f.write_str("O"));
                } else if self.square.contains(&p) {
                    result = result.and_then(|_| f.write_str("#"));
                } else {
                    result = result.and_then(|_| f.write_str("."));
                }
            }
            result = result.and_then(|_| f.write_str("\r\n"));
        }

        result
    }
}

impl FromStr for Dish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<char> = s.parse().unwrap();

        let height = grid.height();
        let width = grid.width();

        let mut round: HashSet<Point> = HashSet::new();
        let mut square: HashSet<Point> = HashSet::new();
        let plane: Plane = (&grid).into();

        for row in 0..height {
            let y = height - row - 1;
            for x in 0..width {
                let c = grid.rows[row][x];
                match c {
                    'O' => {
                        round.insert((x as i64, y as i64).into());
                    }
                    '#' => {
                        square.insert((x as i64, y as i64).into());
                    }
                    '.' => {}
                    _ => panic!("bad char"),
                }
            }
        }

        Ok(Dish {
            round,
            square,
            plane,
        })
    }
}

fn tilt(d: Dish, tilt: &Tilt) -> Dish {
    let mut moved: HashSet<Point> = HashSet::new();

    match &tilt {
        Tilt::East => {
            for col in 0..=d.plane.bottom_right.x {
                let x = d.plane.width() - col - 1;
                for row in 0..=d.plane.top_left.y {
                    let y = d.plane.height() - row - 1;
                    let p: Point = (x, y).into();
                    apply_tilt(&d, &p, &tilt, &mut moved);
                }
            }
        }
        Tilt::West => {
            for x in 0..=d.plane.bottom_right.x {
                for row in 0..=d.plane.top_left.y {
                    let y = d.plane.height() - row - 1;
                    let p: Point = (x, y).into();
                    apply_tilt(&d, &p, &tilt, &mut moved);
                }
            }
        }
        Tilt::North => {
            for row in 0..=d.plane.top_left.y {
                let y = d.plane.height() - row - 1;
                for x in 0..=d.plane.bottom_right.x {
                    let p: Point = (x, y).into();
                    apply_tilt(&d, &p, &tilt, &mut moved);
                }
            }
        }
        Tilt::South => {
            for row in 0..=d.plane.top_left.y {
                let y = row;
                for x in 0..=d.plane.bottom_right.x {
                    let p: Point = (x, y).into();
                    apply_tilt(&d, &p, &tilt, &mut moved);
                }
            }
        }
    }

    if moved.len() != d.round.len() {
        panic!("{} != {} after {:?}", moved.len(), d.round.len(), tilt);
    }

    Dish {
        plane: d.plane,
        round: moved,
        square: d.square,
    }
}

fn apply_tilt(d: &Dish, p: &Point, tilt: &Tilt, moved: &mut HashSet<Point>) {
    if d.round.contains(&p) {
        let mut previous = p.clone();
        loop {
            let moved_up = previous.transform(&tilt.into());
            if moved_up.within(&d.plane) {
                if moved.contains(&moved_up) {
                    // collided with another rock already moved
                    moved.insert(previous);
                    break;
                }

                if d.square.contains(&moved_up) {
                    // collided with square
                    moved.insert(previous);
                    break;
                }

                previous = moved_up;
            } else {
                // gone off the plane
                moved.insert(previous);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(136, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(64, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let txt = include_str!("input.test.txt");
        let dish: Dish = txt.parse().unwrap();

        let width = dish.plane.width();
        let height = dish.plane.height();

        assert_eq!(10, width);
        assert_eq!(10, height);

        assert_eq!(true, dish.round.contains(&(0, 9).into()));

        print(&dish);
    }

    #[test]
    fn test_tilt() {
        let txt = include_str!("input.test.txt");
        let dish: Dish = txt.parse().unwrap();
        let tilted = tilt(dish, &Tilt::North);
        // print(&tilted);

        let txt = include_str!("input.test.tilted.txt");
        let tilted_expected: Dish = txt.parse().unwrap();
        assert_eq!(tilted_expected, tilted);

        let mut counts: Vec<_> = tilted_expected
            .round
            .iter()
            .map(|rock| rock.y + 1)
            .collect();
        counts.sort();
        println!("counts\n{:?}", counts);
        println!("{}", counts.iter().sum::<i64>());
    }

    #[test]
    fn test_tilt_west() {
        let txt = include_str!("input.test.txt");
        let dish: Dish = txt.parse().unwrap();
        let tilted = tilt(dish, &Tilt::South);
        print(&tilted);
    }

    #[test]
    fn test_cycles() {
        let txt = include_str!("input.test.txt");
        let dish: Dish = txt.parse().unwrap();

        let dish = dish.cycle();

        let txt = include_str!("output.test.1.txt");
        let tilted_expected: Dish = txt.parse().unwrap();
        print(&dish);
        assert_eq!(tilted_expected, dish);

        let dish = dish.cycle();

        let txt = include_str!("output.test.2.txt");
        let tilted_expected: Dish = txt.parse().unwrap();
        print(&dish);
        assert_eq!(tilted_expected, dish);

        let dish = dish.cycle();

        let txt = include_str!("output.test.3.txt");
        let tilted_expected: Dish = txt.parse().unwrap();
        print(&dish);
        assert_eq!(tilted_expected, dish);
    }

    fn print(d: &Dish) {
        let width = d.plane.width();
        let height = d.plane.height();
        for row in 0..height {
            let y = height - row - 1;
            for x in 0..width {
                let p: Point = (x as i64, y as i64).into();
                if d.round.contains(&p) {
                    print!("O");
                } else if d.square.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
