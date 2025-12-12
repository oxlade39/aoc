use core::{fmt, str};
use std::{fmt::Debug, hash::Hash, str::FromStr, time::Instant, usize};

use aoclib::{
    grid::{Flip, FromChar, Grid, GridPosition},
    input, timing,
};
use hashbrown::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let i: PuzzleInput = txt.parse().unwrap();

    i.box_targets
        .iter()
        .filter(|b| {
            let BoxTarget {
                width,
                length,
                quantities,
            } = b;
            let total_size: usize = quantities
                .iter()
                .enumerate()
                .map(|(index, n)| {
                    let p = &i.presents[index];
                    let size = p.used_space().len();
                    size * n
                })
                .sum();
            total_size < (width * length)
        })
        .count()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Present {
    shape: Grid<Tile>,
}

impl Present {
    fn used_space(&self) -> HashSet<GridPosition> {
        self.shape
            .position_itr()
            .filter(|(_p, t)| t == &&Tile::Taken)
            .map(|(p, _)| p)
            .collect()
    }
}

impl Hash for Present {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.shape.rows.hash(state);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Space,
    Taken,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Taken => write!(f, "#"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space => write!(f, "."),
            Self::Taken => write!(f, "#"),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Space
    }
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '.' => Ok(Tile::Space),
            '#' => Ok(Tile::Taken),
            other => Err(format!("base tile {}", other)),
        }
    }
}

impl Flip for Tile {
    fn flip(&self) -> Self {
        *self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PuzzleInput {
    presents: Vec<Present>,
    box_targets: Vec<BoxTarget>,
}

#[derive(Debug, PartialEq, Eq)]
struct BoxTarget {
    width: usize,
    length: usize,
    quantities: Vec<usize>,
}

impl FromStr for BoxTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let sizing = parts.next().expect("sizing");
        let (left, right) = sizing.split_once("x").expect("WxL");
        let width: usize = left.parse().expect("width");
        let length: usize = right.replace(":", "").parse().expect("length");
        let quantities = parts.map(|i| i.parse().expect("count")).collect();
        Ok(BoxTarget {
            width,
            length,
            quantities,
        })
    }
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Present {
            shape: s.parse().unwrap(),
        })
    }
}

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks: Vec<_> = input::empty_line_chunks(s).collect();
        let presents = &chunks[0..chunks.len() - 1];
        let targets = &chunks[chunks.len() - 1];

        let presents = presents
            .iter()
            .map(|&p_chunk| {
                let (_, p_chunk) = p_chunk
                    .split_once(&format!(":{}", input::NEW_LINE))
                    .expect(":");
                p_chunk.parse().expect("present grid")
            })
            .collect();

        let box_targets = targets
            .lines()
            .map(|l| l.parse().expect("box target"))
            .collect();

        Ok(PuzzleInput {
            presents,
            box_targets,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_box_target() {
        let s = "4x4: 0 0 0 0 2 0";
        let b: BoxTarget = s.parse().unwrap();
        assert_eq!(
            BoxTarget {
                width: 4,
                length: 4,
                quantities: vec![0, 0, 0, 0, 2, 0]
            },
            b
        );
        let s = "12x5: 1 0 1 0 2 2";
        let b: BoxTarget = s.parse().unwrap();
        assert_eq!(
            BoxTarget {
                width: 12,
                length: 5,
                quantities: vec![1, 0, 1, 0, 2, 2]
            },
            b
        );
    }

    #[test]
    fn test_parse_input_pt1() {
        let test_input = include_str!("input.test.txt");
        let parsed: Result<PuzzleInput, _> = test_input.parse();
        // println!("{:?}", parsed);
        assert!(parsed.is_ok());
        // for p in parsed.unwrap().presents {
        //     println!("{:?}", p.shape);
        //     println!("{}", p.shape);
        // }
    }
}
