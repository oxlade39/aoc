use core::str;
use std::{time::Instant, usize};

use aoclib::{grid::{Grid, GridPosition}, timing};
use hashbrown::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let g: Grid<usize> = txt.parse().unwrap();
    let mut starts = HashSet::new();
    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);
            if *g.at(&p) == 0 {
                starts.insert(p);
            }
        }
    }


    let mut unique: HashMap<GridPosition, HashSet<GridPosition>> = HashMap::new();
    for start in starts {
        let trailhead = next(start, &g, &mut vec![]);
        if !trailhead.is_empty() {
            let ends = trailhead.iter().map(|path| path.last().unwrap().clone()).collect();
            match unique.get_mut(&start) { Some(existing) => {
                existing.extend(ends);
            } _ => {
                unique.insert(start, ends);
            }}
        }
    }
    unique.values().map(|p| p.len()).sum()
    
}

fn part2(txt: &str) -> usize {
    let g: Grid<usize> = txt.parse().unwrap();
    let mut starts = HashSet::new();
    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);
            if *g.at(&p) == 0 {
                starts.insert(p);
            }
        }
    }

    let mut paths: HashMap<GridPosition, Vec<Vec<GridPosition>>> = HashMap::new();

    for start in starts {
        let trailhead = next(start, &g, &mut vec![]);
        if !trailhead.is_empty() {
            paths.insert(start, trailhead);
        }
    }

    paths.values().map(|trailhead| trailhead.len()).sum()
}

fn next(
    p: GridPosition,
    g: &Grid<usize>,
    path: &mut Vec<GridPosition>,
) -> Vec<Vec<GridPosition>> {

    let height = *g.at(&p) as i64;

    if let Some(prev) = path.last() {
        if height - *g.at(prev) as i64 != 1 {
            return vec![];
        }
    }
    
    path.push(p.clone());

    if height == 9 {        
        return vec![path.clone()];
    }

    let mut all_combinations: Vec<Vec<GridPosition>> = Vec::new();

    if p.col != 0 {
        let left = next(p.left(), g, &mut path.clone());
        for child in left {
            if !child.is_empty() {
                all_combinations.push(child);
            }
        }
    }
    if p.col != g.width() - 1 {
        let right = next(p.right(), g, &mut path.clone());
        for child in right {
            if !child.is_empty() {
                all_combinations.push(child);
            }
        }
    }
    if p.row != 0 {
        let up = next(p.up(), g, &mut path.clone());
        for child in up {
            if !child.is_empty() {
                all_combinations.push(child);
            }
        }
    }
    if p.row != g.height() - 1 {
        let down = next(p.down(), g, &mut path.clone());
        for child in down {
            if !child.is_empty() {
                all_combinations.push(child);
            }
        }
    }
    all_combinations
}

#[cfg(test)]
mod tests {
    use aoclib::shortest_path::{Neighbours, NonDiagonalNeighbours};

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
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(36, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(746, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(81, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1541, part2(test_input));
    }
}
