use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{grid::{FromChar, Grid, GridPosition}, input, timing};
use hashbrown::HashSet;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let g: Grid<Tile> = txt.parse().unwrap();
    let mut count = 0;
    for row in 0..g.height() {
        for col in 0..g.width() {
            let pos = GridPosition::new(col, row); 
            let tile = g.at(&pos);
            if tile == &Tile::Space {
                continue;
            }
            let mut neighbours = HashSet::new();
            g.left_from(pos).skip(1).take(1).for_each(|(p, n)| { 
                if n == &Tile::Paper {
                    neighbours.insert(p);
                }
            } );
            g.right_from(pos).skip(1).take(1).for_each(|(p, n)| { 
                if n == &Tile::Paper {
                    neighbours.insert(p); 
                }
            } );
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
            } );
            g.down_from(pos).skip(1).take(1).for_each(|(down_pos, n)| { 
                if n == & Tile::Paper {
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
            } );
            if neighbours.len() < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(txt: &str) -> i64 {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Paper,
    Space
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '@' => Ok(Tile::Paper),
            '.' => Ok(Tile::Space),
            other => Err(format!("bad tile {}", other))
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
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
