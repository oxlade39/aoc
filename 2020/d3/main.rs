use std::{
    fmt::{Display, Write},
    time::Instant,
};

use aoclib::grid::{FromChar, Grid, GridPosition};

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
    let g: Grid<Square> = txt.parse().expect("grid");
    sled(1, 3, &g)
}

fn part2(txt: &str) -> usize {
    let g: Grid<Square> = txt.parse().expect("grid");
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2

    sled(1, 1, &g) * sled(1, 3, &g) * sled(1, 5, &g) * sled(1, 7, &g) * sled(2, 1, &g)
}

fn sled(row_delta: usize, col_delta: usize, g: &Grid<Square>) -> usize {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut trees_hit = 0;
    loop {
        row += row_delta;
        col += col_delta;

        col = col % g.width();

        if row >= g.height() {
            break;
        }

        if g.at(&GridPosition::new(col, row)) == &Square::Tree {
            trees_hit += 1;
        }
    }
    trees_hit
}

#[derive(Debug, PartialEq, Eq)]
enum Square {
    Open,
    Tree,
}

impl FromChar for Square {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '#' => Ok(Square::Tree),
            '.' => Ok(Square::Open),
            other => Err(format!("bad square {other}")),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Square::Open => '.',
            Square::Tree => '#',
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parsing() {
        let g: Grid<Square> = include_str!("input.test.txt").parse().expect("grid");
        let start: GridPosition = GridPosition::new(0, 0);
        let bottom_right = GridPosition::new(g.width() - 1, g.height() - 1);
        let bottom_left = GridPosition::new(0, g.height() - 1);
        assert_eq!(g.at(&start), &Square::Open);
        assert_eq!(g.at(&bottom_right), &Square::Tree);
        assert_eq!(g.at(&bottom_left), &Square::Open);
        assert_eq!(g.at(&bottom_left.up()), &Square::Tree);
    }

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(7, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(189, part1(test_input));
    }

    #[test]
    fn sample_input_pt2() {
        assert_eq!(336, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1718180100, part2(test_input));
    }
}
