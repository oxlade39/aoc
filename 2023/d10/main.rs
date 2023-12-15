use std::{collections::HashSet, i64, str::FromStr, time::Instant};

use aoclib::cartesian::{Plane, Point};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let g: Grid = txt.parse().expect("valid grid");
    let mut start_pos = None;

    let height = g.height();
    let width = g.width();

    for row in 0..height {
        let y = row;
        for x in 0..width {
            if g.0[y][x] == Tile::Start {
                start_pos = Some(Point {
                    x: x as i64,
                    y: y as i64,
                });
                break;
            }
        }
    }

    let plane: Plane = (&g).into();
    let start_pos = start_pos.expect("must be a start");

    let mut seen = HashSet::new();
    path(&g, start_pos, &plane, &mut seen);

    seen.len() / 2
}

fn part2(txt: &str) -> usize {
    let g: Grid = txt.parse().expect("valid grid");
    let mut start_pos = None;

    let height = g.height();
    let width = g.width();

    for row in 0..height {
        let y = row;
        for x in 0..width {
            if g.0[y][x] == Tile::Start {
                start_pos = Some(Point {
                    x: x as i64,
                    y: y as i64,
                });
                break;
            }
        }
    }

    let plane: Plane = (&g).into();
    let start_pos = start_pos.expect("must be a start");

    let mut seen = HashSet::new();
    path(&g, start_pos, &plane, &mut seen);

    // check all points
    let mut count = 0;
    for row in 0..height {
        let y = row;
        for x in 0..width {
            let p = Point {
                x: x as i64,
                y: y as i64,
            };
            // if this point isn't on the path
            // check if it's inside or outside the path
            if !seen.contains(&p) {
                let mut edge_crosses = 0;

                // need to keep track of how we first crossed the edge
                // L---7   => crosses once
                // F---J   => crossed twice
                let mut start_edge = None;

                for check_x in x..width {
                    let check_point: Point = (check_x as i64, y as i64).into();
                    let pipe_type = g.at(&check_point);
                    if seen.contains(&check_point) {
                        match start_edge {
                            Some(&open) => {
                                match (open, pipe_type) {
                                    (Tile::SouthAndEastBend, &Tile::SouthAndWestBend) => {
                                        edge_crosses += 2;
                                        start_edge = None;
                                    }
                                    (Tile::NorthAndEastBend, &Tile::NorthAndWestBend) => {
                                        edge_crosses += 2;
                                        start_edge = None;
                                    }
                                    (_, &Tile::Horizonal) => {
                                        // start_edge = None;
                                    }
                                    _ => {
                                        edge_crosses += 1;
                                        start_edge = None;
                                    }
                                }
                            }
                            _ => {
                                match pipe_type {
                                    Tile::SouthAndEastBend => {
                                        start_edge = Some(pipe_type);
                                    }
                                    Tile::NorthAndEastBend => {
                                        start_edge = Some(pipe_type);
                                    }
                                    Tile::Horizonal => {
                                        // start_edge = None;
                                    }
                                    _ => {
                                        edge_crosses += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if edge_crosses % 2 != 0 {
                    // odd, so inside the edges
                    count += 1;
                    // println!("{:?} inside with {:?} and {:?}", p, edge_crosses, g.at(&p));
                }
            }
        }
    }

    count
}

fn next<'a>(g: &Grid, next: Point, plane: &Plane, seen: &mut HashSet<Point>) -> Option<Point> {
    if seen.contains(&next) {
        return None;
    }
    seen.insert(next.clone());

    let tile = g.at(&next);
    let connections = tile.connects();
    // println!("{:?} connects on {:?}", next, connections);
    for c in connections {
        match c {
            Connects::Down => {
                let down_point = next.transform(&(0, -1).into());
                if down_point.within(plane) && !seen.contains(&down_point) {
                    let down = g.at(&down_point);
                    if down.connects().contains(&Connects::Up) {
                        return Some(down_point);
                    }
                }
            }
            Connects::Up => {
                let up_point = next.transform(&(0, 1).into());
                if up_point.within(&plane) && !seen.contains(&up_point) {
                    let up = g.at(&up_point);
                    if up.connects().contains(&Connects::Down) {
                        return Some(up_point);
                    }
                }
            }
            Connects::Left => {
                let left_point = next.transform(&(-1, 0).into());
                if left_point.within(&plane) && !seen.contains(&left_point) {
                    let left = g.at(&left_point);
                    if left.connects().contains(&Connects::Right) {
                        return Some(left_point);
                    }
                }
            }
            Connects::Right => {
                let right_point = next.transform(&(1, 0).into());
                if right_point.within(&plane) && !seen.contains(&right_point) {
                    let right = g.at(&right_point);
                    if right.connects().contains(&Connects::Left) {
                        return Some(right_point);
                    }
                }
            }
        }
    }
    None
}

fn path(g: &Grid, start: Point, plane: &Plane, seen: &mut HashSet<Point>) {
    let mut n = start;
    while let Some(x) = next(g, n.clone(), plane, seen) {
        n = x;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    // | is a vertical pipe connecting north and south.
    Vertical,

    // - is a horizontal pipe connecting east and west.
    Horizonal,

    // L is a 90-degree bend connecting north and east.
    NorthAndEastBend,

    // J is a 90-degree bend connecting north and west.
    NorthAndWestBend,

    // 7 is a 90-degree bend connecting south and west.
    SouthAndWestBend,

    // F is a 90-degree bend connecting south and east.
    SouthAndEastBend,

    // . is ground; there is no pipe in this tile.
    Ground,

    // S is the starting position of the animal; there is a pipe on this
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Connects {
    Left,
    Right,
    Up,
    Down,
}

impl Tile {
    fn connects(&self) -> HashSet<Connects> {
        match self {
            Tile::Start => HashSet::from_iter(vec![
                Connects::Left,
                Connects::Right,
                Connects::Up,
                Connects::Down,
            ]),
            Tile::Ground => HashSet::new(),
            Tile::NorthAndEastBend => HashSet::from_iter(vec![Connects::Up, Connects::Right]),
            Tile::NorthAndWestBend => HashSet::from_iter(vec![Connects::Up, Connects::Left]),
            Tile::SouthAndEastBend => HashSet::from_iter(vec![Connects::Down, Connects::Right]),
            Tile::SouthAndWestBend => HashSet::from_iter(vec![Connects::Down, Connects::Left]),
            Tile::Vertical => HashSet::from_iter(vec![Connects::Up, Connects::Down]),
            Tile::Horizonal => HashSet::from_iter(vec![Connects::Left, Connects::Right]),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn at(&self, p: &Point) -> &Tile {
        &self.0[p.y as usize][p.x as usize]
    }
}

impl From<&Grid> for Plane {
    fn from(value: &Grid) -> Self {
        let max_y = (value.0.len() - 1) as i64;
        let max_x = (value.0[0].len() - 1) as i64;
        Plane {
            top_left: (0, max_y).into(),
            bottom_right: Point { x: max_x, y: 0 },
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '|' => Tile::Vertical,
                        '-' => Tile::Horizonal,
                        'L' => Tile::NorthAndEastBend,
                        'J' => Tile::NorthAndWestBend,
                        '7' => Tile::SouthAndWestBend,
                        'F' => Tile::SouthAndEastBend,
                        '.' => Tile::Ground,
                        'S' => Tile::Start,
                        _ => unreachable!("bad tile"),
                    })
                    .collect_vec()
            })
            .collect_vec();
        grid.reverse();
        Ok(Self(grid))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(4, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(4, part2(include_str!("input.test2.txt")));
    }

    #[test]
    fn test_parse_grid() {
        let input = include_str!("input.test.txt");
        let g = input.parse::<Grid>();

        assert_eq!(true, g.is_ok());
    }

    #[test]
    fn test_next() {
        let input = include_str!("input.test.txt");
        let g = input.parse::<Grid>().unwrap();

        let plane: Plane = (&g).into();
        let p_one = (1, 3).into();

        let mut seen = HashSet::new();
        path(&g, p_one, &plane, &mut seen);
        println!("p: {:?}", seen);
    }
}
