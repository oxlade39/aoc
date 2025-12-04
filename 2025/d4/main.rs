use core::str;
use std::time::Instant;

use aoclib::{
    grid::{FromChar, Grid, GridPosition},
    timing,
};
use hashbrown::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let g: Grid<Tile> = txt.parse().unwrap();
    accessible(&g).len()
}

fn part2(txt: &str) -> usize {
    let mut g: Grid<Tile> = txt.parse().unwrap();
    let mut count = 0;

    let mut next = accessible(&g);
    while !next.is_empty() {
        count += next.len();

        for item in next.drain() {
            *g.at_mut(&item) = Tile::Space;
        }
        next = accessible(&g);
    }

    count
}

fn paper_neighbours(g: &Grid<Tile>, pos: GridPosition) -> HashSet<GridPosition> {
    let tile = g.at(&pos);
    if tile == &Tile::Space {
        return HashSet::new();
    }
    let mut neighbours = HashSet::new();
    g.left_from(pos).skip(1).take(1).for_each(|(p, n)| {
        if n == &Tile::Paper {
            neighbours.insert(p);
        }
    });
    g.right_from(pos).skip(1).take(1).for_each(|(p, n)| {
        if n == &Tile::Paper {
            neighbours.insert(p);
        }
    });
    g.up_from(pos).skip(1).take(1).for_each(|(up_pos, n)| {
        if n == &Tile::Paper {
            neighbours.insert(up_pos);
        }
        g.left_from(up_pos).skip(1).take(1).for_each(|(p, n)| {
            if n == &Tile::Paper {
                neighbours.insert(p);
            }
        });
        g.right_from(up_pos).skip(1).take(1).for_each(|(p, n)| {
            if n == &Tile::Paper {
                neighbours.insert(p);
            }
        });
    });
    g.down_from(pos).skip(1).take(1).for_each(|(down_pos, n)| {
        if n == &Tile::Paper {
            neighbours.insert(down_pos);
        }
        g.left_from(down_pos).skip(1).take(1).for_each(|(p, n)| {
            if n == &Tile::Paper {
                neighbours.insert(p);
            }
        });
        g.right_from(down_pos).skip(1).take(1).for_each(|(p, n)| {
            if n == &Tile::Paper {
                neighbours.insert(p);
            }
        });
    });
    neighbours
}

fn accessible(g: &Grid<Tile>) -> HashSet<GridPosition> {
    let mut accessible = HashSet::new();
    for row in 0..g.height() {
        for col in 0..g.width() {
            let pos = GridPosition::new(col, row);
            let tile = g.at(&pos);
            if tile == &Tile::Space {
                continue;
            }
            let neighbours = paper_neighbours(&g, pos);
            if neighbours.len() < 4 {
                accessible.insert(pos);
            }
        }
    }
    accessible
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Paper,
    Space,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '@' => Ok(Tile::Paper),
            '.' => Ok(Tile::Space),
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
        assert_eq!(13, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1460, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(43, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(9243, part2(test_input));
    }
}
