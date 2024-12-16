use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{grid::{FromChar, Grid, GridPosition}, input, shortest_path::{astar, Cost, Heuristic, ManhattenDistanceTo, Neighbours}, timing};
use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let map: Map = txt.parse().unwrap();

    let initial_state = Reindeer { 
        direction: Direction::Right,
        position: map.start,
    };
    let end_state = |r: &Reindeer| r.position == map.end;

    let path = astar(
        &map, 
        &map, 
        &map.end, 
        initial_state, 
        end_state
    );

    path.unwrap().total_cost
}

fn part2(txt: &str) -> usize {
    let map: Map = txt.parse().unwrap();
    let mut visited_map = VisitedMap { map, visited: HashSet::new() };

    let initial_state = Reindeer { 
        direction: Direction::Right,
        position: visited_map.map.start,
    };

    let end = visited_map.map.end.clone();
    let end_state = |r: &Reindeer| r.position == end;

    let mut best_cost = 10000000000000;

    let mut count = 0;

    while let Some(path) = astar(
        &visited_map, 
        &visited_map, 
        &end, 
        initial_state, 
        end_state
    ) {
        let this_path_points: HashSet<GridPosition> = path.path.iter()
            .map(|(p, _c)|&p.position)
            .copied()
            .collect();

        let num_same_points = this_path_points.intersection(&visited_map.visited).count();
        let this_path_cost = path.total_cost;
        let original_cost = this_path_cost - num_same_points;

        if original_cost > best_cost {
            break;
        }
        if num_same_points == this_path_points.len() {
            break;
        }

        dbg!(num_same_points, this_path_cost, original_cost, this_path_points.len());
        println!("");

        count += 1;
        if count > 10 {
            break;
        }

        best_cost = original_cost;

        visited_map.visit_all(this_path_points);

    }

    visited_map.visited.len() + 1
}

impl Neighbours<Reindeer> for Map {
    fn neighbours(&self, state: &Reindeer) -> Vec<Reindeer> {
        let mut n: Vec<Reindeer> = Vec::with_capacity(4);
        
        if state.position.col > 0 {
            let left = state.position.left();
            if *self.g.at(&left) != Tile::Wall {
                n.push(Reindeer { 
                    direction: Direction::Left, 
                    position: left, 
                });
            }            
        }

        if state.position.col < self.g.width() - 1 {
            let right = state.position.right();
            if *self.g.at(&right) != Tile::Wall {
                n.push(Reindeer { 
                    direction: Direction::Right, 
                    position: right, 
                });
            }            
        }

        if state.position.row > 0 {
            let up = state.position.up();
            if *self.g.at(&up) != Tile::Wall {
                n.push(Reindeer { 
                    direction: Direction::Up, 
                    position: up, 
                });
            }            
        }

        if state.position.row < self.g.height() - 1 {
            let down = state.position.down();
            if *self.g.at(&down) != Tile::Wall {
                n.push(Reindeer { 
                    direction: Direction::Down, 
                    position: down, 
                });
            }            
        }
        n
    }
}

impl Heuristic<Reindeer, usize> for GridPosition {
    fn predict(&self, from: &Reindeer) -> usize {
        ManhattenDistanceTo(*self).predict(&from.position)
    }
}

impl Cost<Reindeer, usize> for Map {
    fn measure(&self, from: &Reindeer, to: &Reindeer) -> usize {
        // neighbours should only ever return positions 1 and up to 90deg
        let mut c = 1;
        if from.direction != to.direction {
            c += 1000;
        }
        c
    }
}

struct VisitedMap {
    map: Map,
    visited: HashSet<GridPosition>,
}

impl VisitedMap {
    fn visit_all(&mut self, positions: HashSet<GridPosition>) {
        self.visited.extend(positions);
    }
}

impl Cost<Reindeer, usize> for VisitedMap {
    fn measure(&self, from: &Reindeer, to: &Reindeer) -> usize {
        let mut c = 1;
        if from.direction != to.direction {
            c += 1000;
        }
        // don't travel on a previous path
        if self.visited.contains(&to.position) {
            c += 1;
        }
        c
    }
}

impl Neighbours<Reindeer> for VisitedMap {
    fn neighbours(&self, state: &Reindeer) -> Vec<Reindeer> {
        self.map.neighbours(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Reindeer {
    direction: Direction,
    position: GridPosition, 
}

struct Map { 
    g: Grid<Tile>,
    start: GridPosition,
    end: GridPosition,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g: Grid<Tile> = s.parse().unwrap();

        let mut start = None;
        let mut end = None;

        for row in 0..g.height() {
            for col in 0..g.width() {
                let p = GridPosition::new(col, row);
                let t = g.at(&p);
                if *t == Tile::Start {
                    start = Some(p)
                } else if *t == Tile::End {
                    end = Some(p)
                }
            }
        }

        Ok(Map { g, start: start.expect("start"), end: end.expect("end") })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Start,
    End,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Space),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            other => Err(format!("bad tile '{other}'"))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(11048, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(64, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
