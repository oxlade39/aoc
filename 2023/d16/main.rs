use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    str::FromStr,
    time::Instant, default,
};

use aoclib::{input::{FromChar, Flip, Grid}, astar::Cost, cartesian::{Point, Transform}};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let c: Contraption = txt.parse().unwrap();
    let mut path = vec![];
    let start_point = (0, c.tiles.height() as i64 - 1).into();
    let paths = step(&c, Direction::Right, start_point, &mut path, &mut HashSet::new());
    collect_energised(&c, paths).len()
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum Tile {
    #[default]
    Space,
    DiagonalRight,
    DiagonalLeft,
    UpDown,
    LeftRight,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '.' => Ok(Tile::Space),
            '/' => Ok(Tile::DiagonalRight),
            '\\' => Ok(Tile::DiagonalLeft),
            '|' => Ok(Tile::UpDown),
            '-' => Ok(Tile::LeftRight),
            _ => Err("no mapping".to_owned())
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Space => f.write_str("."),
            Tile::DiagonalRight => f.write_str("/"),
            Tile::DiagonalLeft => f.write_str("\\"),
            Tile::UpDown => f.write_str("|"),
            Tile::LeftRight => f.write_str("-"),
        }
    }
}

impl Flip for Tile {
    fn flip(&self) -> Self {
        match self {
            Tile::DiagonalLeft => Tile::DiagonalRight,
            Tile::DiagonalRight => Tile::DiagonalLeft,
            other => other.clone(),
        }
    }
}

struct Contraption {
    tiles: Grid<Tile>
}

impl FromStr for Contraption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Grid<Tile>>().map(|g| Contraption { tiles: g.flip() })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn apply(&self, p: &Point) -> Point {
        match self {
            Direction::Up => p.transform(&(0, 1).into()),
            Direction::Down => p.transform(&(0, -1).into()),
            Direction::Left => p.transform(&(-1, 0).into()),
            Direction::Right => p.transform(&(1, 0).into()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => f.write_str("^"),
            Direction::Down => f.write_str("v"),
            Direction::Left => f.write_str("<"),
            Direction::Right => f.write_str(">"),
        }
    }
}

fn step(
    contraption: &Contraption,
    direction: Direction, 
    current: Point, 
    path: &mut Vec<Point>,    
    seen: &mut HashSet<(Direction, Point)>
) -> Vec<Point> {

    if !seen.insert((direction.clone(), current.clone())) {
        return path.clone();
    }

    let g = &contraption.tiles;
    if !current.within(&g.into()) {
        return path.clone();
    }

    let current_tile = &contraption.tiles.rows[current.y as usize][current.x as usize];
    path.push(current.clone());
    // print_path_and_dir(contraption, &current, &direction, path.clone());

    match current_tile {
        Tile::Space => {            
            let next = direction.apply(&current);
            let p = step(contraption, direction, next, path, seen);
            return p;
        },
        Tile::DiagonalRight => {
            let next_dir: Direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            let next = next_dir.apply(&current);
            let p = step(contraption, next_dir.clone(), next.clone(), path, seen);
            return p;
        },
        Tile::DiagonalLeft => {
            let next_dir: Direction = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            let next = next_dir.apply(&current);
            let p = step(contraption, next_dir.clone(), next.clone(), path, seen);
            return p;
        },
        Tile::UpDown => {
            match direction {
                Direction::Up => {
                    let next = direction.apply(&current);
                    let p = step(contraption, direction.clone(), next.clone(), path, seen);
                    return p;
                },
                Direction::Down => {
                    let next = direction.apply(&current);
                    let p = step(contraption, direction.clone(), next.clone(), path, seen);
                    return p;
                },
                Direction::Left | Direction::Right => {
                    let next_dir = Direction::Up;
                    let next = next_dir.apply(&current);
                    let mut ups = step(contraption, next_dir, next.clone(), &mut path.clone(), seen);

                    let next_dir = Direction::Down;
                    let next = next_dir.apply(&current);
                    let downs = step(contraption, next_dir, next.clone(), &mut path.clone(), seen);

                    ups.extend(downs);
                    
                    return ups;
                },
            }
        },
        Tile::LeftRight => {
            match direction {
                Direction::Up | Direction::Down => {
                    let next_dir = Direction::Left;
                    let next = next_dir.apply(&current);
                    let mut lefts = step(contraption, next_dir.clone(), next.clone(), &mut path.clone(), seen);

                    let next_dir = Direction::Right;
                    let next = next_dir.apply(&current);
                    let combined = step(contraption, next_dir.clone(), next.clone(), &mut lefts, seen);

                    return combined;
                },
                Direction::Left | Direction::Right => {
                    let next = direction.apply(&current);
                    let p = step(contraption, direction.clone(), next.clone(), path, seen);
                    return p;
                },
            }
        },
    }
}

fn collect_energised(c: &Contraption, paths: Vec<Point>) -> HashSet<Point> {
    let mut entergised: HashSet<Point> = HashSet::new();

    for p in paths {
        entergised.insert(p);
    }
    entergised
}

fn print_path(c: &Contraption, paths: Vec<Point>) {
    let entergised = collect_energised(c, paths);

    println!("Paths: ");
    for row in 0..c.tiles.height() {
        let y = c.tiles.height() - row - 1;
        for x in 0..c.tiles.width() {
            if entergised.contains(&(x as i64, y as i64).into()) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn print_path_and_dir(c: &Contraption, p: &Point, d: &Direction, paths: Vec<Point>) {
    let current_tile = &c.tiles.rows[p.y as usize][p.x as usize];
    let entergised = collect_energised(c, paths);

    println!("Paths: [{}] ({},{})", current_tile, p.x, p.y);
    for row in 0..c.tiles.height() {
        let y = (c.tiles.height() as i64 - row as i64) - 1;
        for x in 0..c.tiles.width() {
            let print_point = (x as i64, y as i64).into();            
            if p == &print_point {
                print!("{}", d);
            } else if entergised.contains(&print_point) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(46, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let txt = include_str!("input.test.txt");
        let c: Grid<char> = txt.parse().unwrap();
        println!("{}", c);

        let c: Contraption = txt.parse().unwrap();
        println!("");
        println!("{}", c.tiles);


        println!("{:?}", c.tiles.rows[9]);

        assert_eq!(Tile::Space, c.tiles.rows[9][0]);
        assert_eq!(Tile::UpDown, c.tiles.rows[9][1]);
        assert_eq!(Tile::LeftRight, c.tiles.rows[2][1]);
    }

    #[test]
    fn test_step() {
        let txt = include_str!("input.test.txt");
        let c: Contraption = txt.parse().unwrap();

        println!("Contraption:\n{}", c.tiles.flip());

        let mut path = vec![];
        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let paths = step(&c, Direction::Right, start_point, &mut path, &mut HashSet::new());
        print_path(&c, paths);        
    }

    #[test]
    fn test_point_transpose() {
        let from: Point = (5, 2).into();
        let expected: Point = (5, 3).into();
        let up = Direction::Up.apply(&from);

        assert_eq!(expected, up);
    }

}
