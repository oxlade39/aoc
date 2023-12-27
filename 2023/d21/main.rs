use std::{
    collections::HashSet,
    fmt::{Display, Write},
    time::Instant,
};

use aoclib::grid::{FromChar, Grid, GridPosition};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let map = Map(txt.parse().unwrap());
    map.steps(64).len()
}

fn part2(txt: &str) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Map(Grid<Position>);

impl Map {
    fn start_pos(&self) -> Option<GridPosition> {
        for row in 0..self.0.height() {
            for col in 0..self.0.width() {
                let pos = GridPosition::new(col, row);
                if self.0.at(&pos) == &Position::Start {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn can_walk_to(&self, p: &GridPosition) -> bool {
        match self.0.at(p) {
            Position::Start => true,
            Position::GardenPlot => true,
            Position::Rock => false,
        }
    }

    fn step(&self, current: HashSet<GridPosition>) -> HashSet<GridPosition> {
        let mut next: HashSet<GridPosition> = HashSet::new();
        let mut position_itr = current.into_iter();

        while let Some(p) = position_itr.next() {
            if p.col > 0 {
                next.insert(p.left());
            }
            if p.col < self.0.width() {
                next.insert(p.right());
            }
            if p.row > 0 {
                next.insert(p.up());
            }
            if p.row < self.0.height() {
                next.insert(p.down());
            }
        }

        next.into_iter().filter(|p| self.can_walk_to(p)).collect()
    }

    fn steps(&self, num: usize) -> HashSet<GridPosition> {
        let start: HashSet<GridPosition> = self.start_pos().into_iter().collect();

        (0..num)
            .into_iter()
            .fold(start, |accum, _| self.step(accum))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Start,
    GardenPlot,
    Rock,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Start => f.write_char('S'),
            Position::GardenPlot => f.write_char('.'),
            Position::Rock => f.write_char('#'),
        }
    }
}

impl FromChar for Position {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'S' => Ok(Position::Start),
            '.' => Ok(Position::GardenPlot),
            '#' => Ok(Position::Rock),
            other => Err(format!("bad char: {}", other)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_example_p1() {
        let txt = include_str!("input.test.txt");
        let map = Map(txt.parse().unwrap());
        assert_eq!(16, map.steps(6).len());
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_pt1_steps() {
        let txt = include_str!("input.test.txt");
        let map = Map(txt.parse().unwrap());

        let positions = map.start_pos().into_iter().collect();
        let next = map.step(positions);
        assert_eq!(2, next.len());

        let next = map.step(next);
        assert_eq!(4, next.len());
    }
}
