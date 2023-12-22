use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
    time::Instant,
};

use aoclib::{
    cartesian::Point,
    grid::{Flip, FromChar, Grid},
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let c: Contraption = txt.parse().unwrap();
    let start_point = (0, c.tiles.height() as i64 - 1).into();
    let mut points = HashSet::new();
    step(
        &c,
        Direction::Right,
        start_point,
        &mut points,
        &mut HashSet::new(),
    );
    points.len()
}

fn part2(txt: &str) -> usize {
    let c: Contraption = txt.parse().unwrap();

    let mut max = 0;
    for x in 0..c.tiles.width() {
        let start_point = (x as i64, c.tiles.height() as i64 - 1).into();

        let mut points = HashSet::new();
        let mut path_cache = HashSet::new();
        step(
            &c,
            Direction::Down,
            start_point,
            &mut points,
            &mut path_cache,
        );
        max = max.max(points.len());

        let start_point = (x as i64, 0).into();
        let mut points = HashSet::new();
        let mut path_cache = HashSet::new();
        step(&c, Direction::Up, start_point, &mut points, &mut path_cache);
        max = max.max(points.len());
    }

    for y in 0..c.tiles.height() {
        let start_point = (c.tiles.width() as i64 - 1, y as i64).into();

        let mut points = HashSet::new();
        let mut path_cache = HashSet::new();
        step(
            &c,
            Direction::Left,
            start_point,
            &mut points,
            &mut path_cache,
        );
        max = max.max(points.len());

        let start_point = (0, y as i64).into();
        let mut points = HashSet::new();
        let mut path_cache = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut path_cache,
        );
        max = max.max(points.len());
    }
    max
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
            _ => Err("no mapping".to_owned()),
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
    tiles: Grid<Tile>,
}

impl FromStr for Contraption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Grid<Tile>>()
            .map(|g| Contraption { tiles: g.flip() })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
    points: &mut HashSet<Point>,
    seen: &mut HashSet<(Direction, Point)>,
) {
    if !seen.insert((direction.clone(), current.clone())) {
        return;
    }

    let g = &contraption.tiles;
    if !current.within(&g.into()) {
        return;
    }

    let current_tile = &contraption.tiles.rows[current.y as usize][current.x as usize];
    points.insert(current.clone());

    match current_tile {
        Tile::Space => {
            let next = direction.apply(&current);
            let p = step(contraption, direction, next, points, seen);
            return p;
        }
        Tile::DiagonalRight => {
            let next_dir: Direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
            let next = next_dir.apply(&current);
            step(contraption, next_dir.clone(), next.clone(), points, seen);
            return;
        }
        Tile::DiagonalLeft => {
            let next_dir: Direction = match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            let next = next_dir.apply(&current);
            step(contraption, next_dir.clone(), next.clone(), points, seen);
            return;
        }
        Tile::UpDown => match direction {
            Direction::Up => {
                let next = direction.apply(&current);
                step(contraption, direction.clone(), next.clone(), points, seen);
                return;
            }
            Direction::Down => {
                let next = direction.apply(&current);
                step(contraption, direction.clone(), next.clone(), points, seen);
                return;
            }
            Direction::Left | Direction::Right => {
                let next_dir = Direction::Up;
                let next = next_dir.apply(&current);
                step(contraption, next_dir, next.clone(), points, seen);

                let next_dir = Direction::Down;
                let next = next_dir.apply(&current);
                step(contraption, next_dir, next.clone(), points, seen);
                return;
            }
        },
        Tile::LeftRight => match direction {
            Direction::Up | Direction::Down => {
                let next_dir = Direction::Left;
                let next = next_dir.apply(&current);
                step(contraption, next_dir.clone(), next.clone(), points, seen);

                let next_dir = Direction::Right;
                let next = next_dir.apply(&current);
                step(contraption, next_dir.clone(), next.clone(), points, seen);

                return;
            }
            Direction::Left | Direction::Right => {
                let next = direction.apply(&current);
                step(contraption, direction.clone(), next.clone(), points, seen);
                return;
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(46, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(51, part2(include_str!("input.test.txt")));
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

        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let mut points = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut HashSet::new(),
        );
        assert!(points.len() > 0);
    }

    #[test]
    fn test_point_transpose() {
        let from: Point = (5, 2).into();
        let expected: Point = (5, 3).into();
        let up = Direction::Up.apply(&from);

        assert_eq!(expected, up);
    }

    #[test]
    fn test_simple_examples() {
        let c: Contraption = "...".parse().unwrap();
        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let mut points = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut HashSet::new(),
        );
        assert_eq!(3, points.len());

        let c: Contraption = "\
        .|.\n\
        .\\.\n\
        "
        .parse()
        .unwrap();
        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let mut points = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut HashSet::new(),
        );
        assert_eq!(4, points.len());

        let c: Contraption = "\
        .|.\n\
        .\\|\n\
        ...\n\
        "
        .parse()
        .unwrap();
        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let mut points = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut HashSet::new(),
        );
        assert_eq!(6, points.len());
    }

    #[test]
    fn test_repeating_example() {
        let c: Contraption = "\
        .|.\n\
        .\\|\n\
        .\\/\n\
        "
        .parse()
        .unwrap();
        let start_point = (0, c.tiles.height() as i64 - 1).into();
        let mut points = HashSet::new();
        step(
            &c,
            Direction::Right,
            start_point,
            &mut points,
            &mut HashSet::new(),
        );
        assert_eq!(8, points.len());
    }
}
