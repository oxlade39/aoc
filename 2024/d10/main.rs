use core::str;
use std::{time::Instant, usize};

use aoclib::{grid::{self, Grid, GridPosition}, neighbour, shortest_path::{astar, Cost, ManhattenDistanceTo, NonDiagonalNeighbours}, timing};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let g: Grid<usize> = txt.parse().unwrap();

    let neighbours = NonDiagonalNeighbours(&g);
    let mut zeros = HashSet::new();
    let mut nines = HashSet::new();

    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);
            let height = g.at(&p);
            if *height == 0 {
                zeros.insert(p.clone());                
            }
            if *height == 9 {
                nines.insert(p);
            }
        }
    }

    let mut total = 0;
    for start in zeros {
        let mut trailhead_score = 0;
        for end in &nines {
            let cost = ExactlyOne(&g);
            let heuristic = ManhattenDistanceTo(end.clone());
            
            let p = astar(
                &neighbours, 
                &cost, 
                &heuristic, 
                start.clone(), 
                |p| {                    
                    let is_end = p == end;
                    // println!("{:?} == {:?} - {}", p, end, is_end);
                    is_end
                }
            );
            if p.is_some() {
                trailhead_score += 1;
            } else {
                // println!("no path {:?} to {:?}", start, end);
            }
        }
        total += trailhead_score;
    }

    total
}

struct ExactlyOne<'a>(&'a Grid<usize>);

impl<'a> Cost<GridPosition, usize> for ExactlyOne<'a> {
    fn measure(&self, from: &GridPosition, to: &GridPosition) -> usize {        
        let from_height = *self.0.at(from) as i64;
        let to_height = *self.0.at(to) as i64;
        let diff = to_height - from_height;

        if diff != 1 {
            10000000
        } else {
            1
        }
    }
}

fn part2(txt: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use aoclib::shortest_path::Neighbours;

    use crate::*;

    #[test]
    fn test_input_pt1_simpl_neighbours() {
        let test_input = include_str!("input.test.simpl.txt");
        let g: Grid<usize> = test_input.parse().unwrap();
        let neighbours = NonDiagonalNeighbours(&g);
        let p = GridPosition::new(1, 3);
        let all = neighbours.neighbours(&p);
        assert_eq!(3, all.len());
    }


    #[test]
    fn test_input_pt1_simpl() {
        let test_input = include_str!("input.test.simpl.txt");
        assert_eq!(1, part1(test_input));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(36, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(0, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
