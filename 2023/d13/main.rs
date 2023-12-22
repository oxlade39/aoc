use std::time::Instant;

use aoclib::{
    grid::{FromChar, Grid},
    input,
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    input::empty_line_chunks(txt)
        .map(|c| c.parse::<Grid<Tile>>().unwrap())
        .map(|g| score(&g, 0))
        .sum()
}

fn part2(txt: &str) -> usize {
    input::empty_line_chunks(txt)
        .map(|c| c.parse::<Grid<Tile>>().unwrap())
        .map(|g| score(&g, 1))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => f.write_str("."),
            Tile::Rock => f.write_str("#"),
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Ash
    }
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '#' => Ok(Tile::Rock),
            '.' => Ok(Tile::Ash),
            _ => Err(format!("bad char [{}]", c)),
        }
    }
}

fn score(g: &Grid<Tile>, diffs: usize) -> usize {
    let row = symmetry_index(&g.rows, diffs).unwrap_or(0);
    let col = symmetry_index(&g.transpose().rows, diffs).unwrap_or(0);

    row * 100 + col
}

fn symmetry_index(items: &Vec<Vec<Tile>>, allowed: usize) -> Option<usize> {
    for i in 0..(items.len() - 1) {
        let mut left_index = i as i32;
        let mut right_index = i as i32 + 1;

        let mut diffs = 0;
        loop {
            if left_index < 0 || right_index >= items.len() as i32 {
                // out of bounds
                break;
            }

            let left_items = &items[left_index as usize];
            let right_items = &items[right_index as usize];

            diffs += left_items
                .iter()
                .zip(right_items.iter())
                .filter(|(left, right)| left != right)
                .count();

            if diffs > allowed {
                break;
            }

            left_index -= 1;
            right_index += 1;
        }

        if diffs == allowed {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use aoclib::input;

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(405, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(400, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse_1() {
        let input: Vec<_> = input::empty_line_chunks(include_str!("input.test.txt")).collect();
        let g = input[0].parse::<Grid<Tile>>().unwrap();

        for row in g.rows.iter() {
            for col in row {
                print!("{}", col);
            }
            println!("");
        }

        let i = symmetry_index(&g.rows, 0);
        assert_eq!(None, i);
        let i = symmetry_index(&g.transpose().rows, 0);
        assert_eq!(Some(5), i);
    }

    #[test]
    fn test_parse_2() {
        let input: Vec<_> = input::empty_line_chunks(include_str!("input.test.txt")).collect();
        let g = input[1].parse::<Grid<Tile>>().unwrap();

        let i = symmetry_index(&g.rows, 0);
        assert_eq!(Some(4), i);
        let i = symmetry_index(&g.transpose().rows, 0);
        assert_eq!(None, i);
    }

    #[test]
    fn test_score_pt2_1() {
        let input: Vec<_> = input::empty_line_chunks(include_str!("input.test.txt")).collect();
        let g = input[0].parse::<Grid<Tile>>().unwrap();

        let i = symmetry_index(&g.rows, 1);
        assert_eq!(Some(3), i);
        let i = symmetry_index(&g.transpose().rows, 1);
        assert_eq!(None, i);
    }
}
