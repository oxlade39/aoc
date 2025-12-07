use core::str;
use std::{i64, time::Instant};

use aoclib::{
    grid::{FromChar, Grid, GridPosition}, timing,
};
use hashbrown::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let g: Grid<Tile> = txt.parse().unwrap();
    let (p, _start) = g.position_itr().find(|(_, t)| t == &&Tile::Start).unwrap();
    let mut memo = HashSet::new();
    count_beams(p, &g, 0, &mut memo)
}

fn part2(txt: &str) -> i64 {
    let g: Grid<Tile> = txt.parse().unwrap();
    let (p, _start) = g.position_itr().find(|(_, t)| t == &&Tile::Start).unwrap();
    let mut memo: HashMap<GridPosition, i64> = HashMap::new();
    count_timelines(p, &g, &mut memo)
}

fn count_beams(
    p: GridPosition,
    g: &Grid<Tile>,
    count: i64,
    seen: &mut HashSet<GridPosition>,
) -> i64 {
    for (below_p, below) in g.down_from(p).skip(1) {
        if below == &Tile::Splitter {
            let mut split_total = count;
            if let Some((left_p, _left)) = g.left_from(below_p).skip(1).take(1).next() {
                seen.insert(left_p);
                split_total += count_beams(left_p, g, 0, seen);
            }
            if let Some((right_p, _left)) = g.right_from(below_p).skip(1).take(1).next() {
                if seen.insert(right_p) {
                    split_total += count_beams(right_p, g, 1, seen);
                }
            }

            return split_total;
        }
    }

    count
}

fn count_timelines(p: GridPosition, g: &Grid<Tile>, memo: &mut HashMap<GridPosition, i64>) -> i64 {
    if let Some(existing) = memo.get(&p) {
        *existing
    } else {
        for (below_p, below) in g.down_from(p).skip(1) {
            if below == &Tile::Splitter {
                let mut result = 0;
                if let Some((left_p, _left)) = g.left_from(below_p).skip(1).take(1).next() {
                    let timelines = count_timelines(left_p, g, memo);
                    memo.insert(left_p, timelines);
                    result += timelines;
                }
                if let Some((right_p, _right)) = g.right_from(below_p).skip(1).take(1).next() {
                    let timelines = count_timelines(right_p, g, memo);
                    memo.insert(right_p, timelines);
                    result += timelines;
                }
                memo.insert(below_p, result);
                return result;
            }
        }
        memo.insert(p, 1);
        1
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Space,
    Splitter,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'S' => Ok(Tile::Start),
            '.' => Ok(Tile::Space),
            '^' => Ok(Tile::Splitter),
            other => Err(format!("bad tile {}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(21, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1585, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(40, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(16716444407407, part2(test_input));
    }
}
