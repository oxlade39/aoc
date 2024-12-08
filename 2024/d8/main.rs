use core::str;
use std::{i64, time::Instant, usize};

use aoclib::grid::{FromChar, Grid, GridPosition};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!(
        "{:.2}ms",
        (now.elapsed().subsec_nanos() as f32) / 1_000_000 as f32
    );
}

fn part1(txt: &str) -> usize {
    let g: Grid<Tile> = txt.parse().unwrap();

    let mut antenna_positions: HashMap<char, HashSet<GridPosition>> = HashMap::new();
    let mut antenna_positions_inv: HashMap<GridPosition, char> = HashMap::new();

    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);
            let t = g.at(&p);
            match t {
                Tile::Antenna(c) => {
                    antenna_positions_inv.insert(p.clone(), *c);
                    if let Some(existing) = antenna_positions.get_mut(c) {
                        existing.insert(p);
                    } else {
                        antenna_positions.insert(*c, HashSet::from_iter(vec![p]));
                    }
                }
                Tile::Space => {
                    // noop
                }
            }
        }
    }

    let mut antinodes: HashSet<GridPosition> = HashSet::new();
    for (_, positions) in antenna_positions {
        for (left, right) in positions.iter().tuple_combinations() {
            let (a1, a2) = calc_antinodes(left, right);
            if let Some(a1_node) = a1 {
                if a1_node.col < g.width() && a1_node.row < g.height() {
                    antinodes.insert(a1.unwrap());
                }
            }
            if let Some(a2_node) = a2 {
                if a2_node.col < g.width() && a2_node.row < g.height() {
                    antinodes.insert(a2.unwrap());
                }
            }
        }
    }

    // print(&g);
    // println!("");
    // print_with_antinodes(&g, &antinodes);

    antinodes.len()
}

#[allow(dead_code)]
fn print(g: &Grid<Tile>) {
    for row in 0..g.height() {
        for col in 0..g.width() {
            let p =GridPosition::new(col, row);
            let t = g.at(&p);            
            match t {
                Tile::Antenna(c) => print!("{c}"),
                Tile::Space => print!("."),
            }
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_with_antinodes(g: &Grid<Tile>, antinodes: &HashSet<GridPosition>) {
    for row in 0..g.height() {
        for col in 0..g.width() {
            let p =GridPosition::new(col, row);
            let t = g.at(&p);
            if antinodes.contains(&p) {
                print!("#")
            } else {
                match t {
                    Tile::Antenna(c) => print!("{c}"),
                    Tile::Space => print!("."),
                }
            }
        }
        println!("");
    }
}

fn calc_antinodes(a: &GridPosition, b: &GridPosition) -> (Option<GridPosition>, Option<GridPosition>) {
    let col_delta = a.col as i64 - b.col as i64;
    let row_delta = a.row as i64 - b.row as i64;

    let col1 = a.col as i64 + col_delta;
    let row1 = a.row as i64 + row_delta;

    let col2 = b.col as i64 - col_delta;
    let row2 = b.row as i64 - row_delta;

    let mut a1 = None;
    let mut b1 = None;

    if col1 >= 0 && row1 >= 0 {
        a1 = Some(GridPosition::new(col1 as usize, row1 as usize));
    }
    if col2 >= 0 && row2 >= 0 {
        b1 = Some(GridPosition::new(col2 as usize, row2 as usize));
    }

    (a1, b1)
}

fn part2(txt: &str) -> usize {
    let g: Grid<Tile> = txt.parse().unwrap();

    let mut antenna_positions: HashMap<char, HashSet<GridPosition>> = HashMap::new();
    let mut antenna_position_set: HashSet<GridPosition> = HashSet::new();

    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);
            let t = g.at(&p);
            match t {
                Tile::Antenna(c) => {
                    antenna_position_set.insert(p.clone());
                    if let Some(existing) = antenna_positions.get_mut(c) {
                        existing.insert(p);
                    } else {
                        antenna_positions.insert(*c, HashSet::from_iter(vec![p]));
                    }
                }
                Tile::Space => {
                    // noop
                }
            }
        }
    }

    let mut antinodes: HashSet<GridPosition> = HashSet::new();
    for (_, positions) in antenna_positions {
        for (left, right) in positions.iter().tuple_combinations() {
            let all = calc_harmonic_antinodes(left, right, &g);
            antinodes = antinodes.union(&all).copied().collect();
        }
    }

    // print(&g);
    // println!("");
    // print_with_antinodes(&g, &antinodes);
    antinodes.union(&antenna_position_set).count()
}

fn calc_harmonic_antinodes(
    a: &GridPosition, 
    b: &GridPosition,
    g: &Grid<Tile>,
) -> HashSet<GridPosition> {
    let col_delta = a.col as i64 - b.col as i64;
    let row_delta = a.row as i64 - b.row as i64;

    let mut results = HashSet::new();

    let mut col = a.col as i64 + col_delta;
    let mut row = a.row as i64 + row_delta;
    loop {
        if col < 0 || col >= g.width() as i64 || row < 0 || row >= g.height() as i64 {
            break;
        }
        results.insert(GridPosition::new(col as usize, row as usize));

        col = col + col_delta;
        row = row + row_delta;        
    }

    // other side
    let mut col = b.col as i64 - col_delta;
    let mut row = b.row as i64 - row_delta;
    loop {
        if col < 0 || col >= g.width() as i64 || row < 0 || row >= g.height() as i64 {
            break;
        }
        results.insert(GridPosition::new(col as usize, row as usize));

        col = col - col_delta;
        row = row - row_delta;        
    }
    results
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Antenna(char),
    Space,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '.' => Ok(Tile::Space),
            other => Ok(Tile::Antenna(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_antinode_positions() {
        let a = GridPosition::new(1, 1);
        let b = GridPosition::new(2, 2);
        let (a1, b1) = calc_antinodes(&a, &b);
        assert_eq!(Some(GridPosition::new(0, 0)), a1);
        assert_eq!(Some(GridPosition::new(3, 3)), b1);

        let a = GridPosition::new(1, 1);
        let b = GridPosition::new(2, 2);
        let (b1, a1) = calc_antinodes(&b, &a);
        assert_eq!(Some(GridPosition::new(0, 0)), a1);
        assert_eq!(Some(GridPosition::new(3, 3)), b1);
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(14, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(247, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(34, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(861, part2(test_input));
    }
}
