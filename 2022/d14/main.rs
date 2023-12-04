use std::{collections::HashSet, str::FromStr};

use aoclib::cartesian::{Point, Transform, Vector};

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    println!("part1: {}", part1);
    let part1 = part2(input);
    println!("part2: {}", part1);
}

#[derive(Debug, PartialEq)]
struct Cave {
    sand: HashSet<Point>,
    rock_paths: Vec<Vector>,
    max_depth: i64,
}

impl Cave {
    fn try_add_point(self, p: Point) -> Result<Self, (Point, Self)> {
        // println!("{:?}", p);
        // we're swapping down for up, since our axis is positive up, negative down
        if p.y >= self.max_depth {
            return Err((p, self));
        }

        let next: [Transform; 3] = [(0, 1).into(), (-1, 1).into(), (1, 1).into()];

        let mut cave = self;

        for t in next {
            let next_p = p.transform(&t);
            let collides_sand = cave.sand.contains(&next_p);
            let collides = collides_sand || cave.rock_paths.iter().any(|path| next_p.on(path));
            if !collides {
                return cave.try_add_point(next_p);
            }
        }

        let new_sand_added = cave.sand.insert(p.clone());
        if !new_sand_added {
            Err((p.clone(), cave))
        } else {
            Ok(cave)
        }
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max_depth = 0;
        let mut vectors: Vec<Vector> = vec![];

        for line in s.lines() {
            let points: Vec<_> = line.split(" -> ").collect();
            for pair in points.windows(2) {
                let left = pair[0];
                let right = pair[1];
                let left_point_str: Vec<_> = left.split(",").collect();
                let right_point_str: Vec<_> = right.split(",").collect();
                let left = Point {
                    x: left_point_str[0].parse().unwrap(),
                    y: left_point_str[1].parse().unwrap(),
                };
                let right = Point {
                    x: right_point_str[0].parse().unwrap(),
                    y: right_point_str[1].parse().unwrap(),
                };
                max_depth = max_depth.max(left.y).max(right.y);
                vectors.push((left, right).into());
            }
        }
        Ok(Cave {
            rock_paths: vectors,
            sand: HashSet::new(),
            max_depth,
        })
    }
}

fn part1(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    let mut count = 0;
    loop {
        let falling_from: Point = (500, 0).into();
        match cave.try_add_point(falling_from) {
            Result::Ok(c) => {
                cave = c;
                count += 1;
            }
            Result::Err(_) => {
                break;
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    cave.rock_paths.push(Vector {
        start: (i64::MIN, cave.max_depth + 2).into(),
        end: (i64::MAX, cave.max_depth + 2).into(),
    });
    cave.max_depth = cave.max_depth + 2;

    let mut count = 0;
    loop {
        let falling_from: Point = (500, 0).into();
        match cave.try_add_point(falling_from) {
            Result::Ok(c) => {
                cave = c;
                count += 1;
            }
            Result::Err(_) => {
                break;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use aoclib::cartesian::Plane;

    use super::*;

    fn draw(c: &Cave, p: &Plane) {
        for y in p.top_left.y..=p.bottom_right.y {
            for x in p.top_left.x..=p.bottom_right.x {
                let point: Point = (x, y).into();
                let is_sand = c.sand.contains(&point);
                let is_wall = !is_sand && c.rock_paths.iter().any(|path| point.on(path));
                if is_wall {
                    print!("#")
                } else if is_sand {
                    print!("O")
                } else {
                    print!(".")
                }
            }
            println!("")
        }
    }

    #[test]
    fn test_parse_cave() {
        let input = include_str!("input.example.txt");
        let expected = Cave {
            sand: HashSet::new(),
            rock_paths: vec![
                // line 1
                ((498, 4).into(), (498, 6).into()).into(),
                ((498, 6).into(), (496, 6).into()).into(),
                // line 2
                ((503, 4).into(), (502, 4).into()).into(),
                ((502, 4).into(), (502, 9).into()).into(),
                ((502, 9).into(), (494, 9).into()).into(),
            ],
            max_depth: 9,
        };

        assert_eq!(Ok(expected), input.parse())
    }

    #[test]
    fn test_example_input() {
        let input = include_str!("input.example.txt");
        let mut cave: Cave = input.parse().unwrap();
        let plane = Plane {
            top_left: (490, 0).into(),
            bottom_right: (505, 9).into(),
        };

        let mut count = 0;
        loop {
            match cave.try_add_point((500, 0).into()) {
                Result::Ok(c) => {
                    cave = c;
                    count += 1;
                    // println!("{} sand added", count);
                }
                Result::Err(_) => {
                    // println!("overflow at {:?}", overflow);
                    break;
                }
            }
            draw(&cave, &plane);
            println!()
        }
        assert_eq!(24, count);
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("input.example.txt");
        let result = part1(input);
        assert_eq!(24, result);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("input.example.txt");
        let result = part2(input);
        assert_eq!(93, result);
    }
}
