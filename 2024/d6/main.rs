use core::str;
use std::time::Instant;

use hashbrown::HashSet;

use aoclib::grid::{FromChar, Grid, GridPosition};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!(
        "{:.2}s",
        now.elapsed().as_secs_f64()
    );
}

fn part1(txt: &str) -> i64 {
    let g: Grid<Tile> = txt.parse().unwrap();
    let guard_position = {
        let mut it = None;
        for row in 0..g.height() {
            for col in 0..g.width() {
                let p = GridPosition::new(col, row);
                if g.at(&p) == &Tile::Guard {
                    it = Some(p);
                }
            }
        }
        it.unwrap()
    };
    path(&g, &guard_position).len() as i64
}

fn part2(txt: &str) -> i64 {
    let mut g: Grid<Tile> = txt.parse().unwrap();

    let guard_position = {
        let mut it = None;
        for row in 0..g.height() {
            for col in 0..g.width() {
                let p = GridPosition::new(col, row);
                if g.at(&p) == &Tile::Guard {
                    it = Some(p);
                }
            }
        }
        it.unwrap()
    };
    let path = path(&g, &guard_position);

    let direction = Direction::Up;
    let visited: HashSet<(GridPosition, Direction)> = HashSet::from_iter(vec![(guard_position, direction)]);
    let mut count = 0;

    for test_pos in path {
        if g.at(&test_pos) == &Tile::Obstruction {
            continue;
        }
        g.rows[test_pos.row][test_pos.col] = Tile::Obstruction;
        if loops(direction, &visited, &guard_position, &g) {
            count += 1;
        }
        g.rows[test_pos.row][test_pos.col] = Tile::Space;
    }
    count
    
}

fn path(g: &Grid<Tile>, guard_position: &GridPosition) -> HashSet<GridPosition> {
    let mut guard_position = guard_position.clone();
    let mut visited: HashSet<GridPosition> = HashSet::from_iter(vec![guard_position]);
    let mut direction = Direction::Up;

    while let Some(next_pos) = direction.make_move(&guard_position) {
        if next_pos.row == g.height() || next_pos.col == g.width() {
            break;
        }
        match g.at(&next_pos) {
            Tile::Obstruction => {
                direction = direction.turn_right();
            },
            Tile::Guard | Tile::Space => {
                guard_position = next_pos;
                visited.insert(guard_position);
            },
        }
    }
    visited
}

fn loops(
    direction: Direction,
    visited: &HashSet<(GridPosition, Direction)>,
    guard_position: &GridPosition,
    g: &Grid<Tile>
) -> bool {
    let mut guard_position = guard_position.clone();
    let mut direction = direction;
    let mut visited = visited.clone();

    while let Some(next_pos) = direction.make_move(&guard_position) {
        if next_pos.row == g.height() || next_pos.col == g.width() {
            break;
        }
        match g.at(&next_pos) {
            Tile::Obstruction => {
                direction = direction.turn_right();
            },
            Tile::Guard | Tile::Space => {
                guard_position = next_pos;
                if !visited.insert((guard_position, direction)) {
                    return true;
                }
            },
        }
    }
    false
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Tile {
    Obstruction,
    Guard,
    Space,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn make_move(&self, p: &GridPosition) -> Option<GridPosition> {
        match self {
            Direction::Up => {
                if p.row == 0 {
                    None
                } else {
                    Some(p.up())
                }                
            },
            Direction::Down => {
                Some(p.down())
            },
            Direction::Left => {
                if p.col == 0 {
                    None
                } else {
                    Some(p.left())
                }                
            },
            Direction::Right => Some(p.right()),
        }
    }
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '.' => Ok(Tile::Space),
            '#' => Ok(Tile::Obstruction),
            '^' => Ok(Tile::Guard),
            other => Err(format!("bad position {}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(41, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(5318, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(6, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1831, part2(test_input));
    }
}
