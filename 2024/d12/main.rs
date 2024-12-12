use core::str;
use std::{str::FromStr, time::Instant, usize};

use aoclib::{grid::{FromChar, Grid, GridPosition}, neighbour, shortest_path::{Neighbours, NonDiagonalNeighbours}, timing};
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
    let garden: Garden = txt.parse().unwrap();

    garden.total_price()
}

fn part2(txt: &str) -> usize {
    let garden: Garden = txt.parse().unwrap();

    garden.total_discount_price()
}

#[derive(Debug, Clone)]
struct Garden(Grid<Plant>, Vec<Plot>);

impl Garden {
    fn total_price(&self) -> usize {
        self.1.iter().map(|plot| plot.price(&self.0)).sum()
    }

    fn total_discount_price(&self) -> usize {
        self.1.iter().map(|plot| plot.discount_price(&self.0)).sum()
    }
}

impl FromStr for Garden {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g: Grid<Plant> = s.parse().unwrap();

        let mut all_positions = HashSet::new();
        for row in 0..g.height() {
            for col in 0..g.width() {
                let p = GridPosition::new(col, row);
                all_positions.insert(p);
            }
        }

        let mut seen = HashSet::new();
        let mut plots: Vec<Plot> = Vec::new();
        for pos in all_positions {
            if seen.contains(&pos) {
                continue;
            }
            let p = g.at(&pos);
            let mut grouping = HashSet::new();
            flood_fill(p, &pos, &g, &mut grouping);
            seen.extend(grouping.clone());
            plots.push(Plot { plant: p.clone(), positions: grouping });
        }
        
        Ok(Garden(g, plots))
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Plant(char);

#[derive(Debug, Clone)]
struct Plot {
    plant: Plant,
    positions: HashSet<GridPosition>,
}

impl Plot {
    fn insert(&mut self, p: GridPosition) {
        self.positions.insert(p);
    }

    fn perimeter(&self, g: &Grid<Plant>) -> usize {
        let mut total = 0;
        for p in &self.positions {
            let plant = g.at(p);

            // up
            if p.row == 0 {
                total += 1;
            } else {
                let up = g.at(&p.up());
                if up != plant {
                    total += 1;
                }
            }

            // down
            if p.row == g.height() - 1 {
                total += 1;
            } else {
                let down = g.at(&p.down());
                if down != plant {
                    total += 1;
                }                
            }

            // left
            if p.col == 0 {
                total += 1;
            } else {
                let left = g.at(&p.left());
                if left != plant {
                    total += 1;
                }
            }

            // right
            if p.col == g.width() - 1 {
                total += 1;
            } else {
                let right = g.at(&p.right());
                if right != plant {
                    total += 1;
                }
            }
        }
        total
    }

    fn sides(&self, g: &Grid<Plant>) -> usize {

        let mut left_sides = HashSet::new();
        let mut top_sides = HashSet::new();
        let mut bottom_sides = HashSet::new();
        let mut right_sides = HashSet::new();

        for p in &self.positions {
            if p.row == 0 {
                top_sides.insert(p.clone());
            } else {
                if !&self.positions.contains(&p.up()) {
                    top_sides.insert(p.clone());
                }
            }
            if p.col == 0 {
                left_sides.insert(p.clone());
            } else {
                if !&self.positions.contains(&p.left()) {
                    left_sides.insert(p.clone());
                }
            }
            if p.row == g.height() - 1 {
                bottom_sides.insert(p.clone());                
            } else {
                if !&self.positions.contains(&p.down()) {
                    bottom_sides.insert(p.clone());
                }
            }
            if p.col == g.width() - 1 {
                right_sides.insert(p.clone());
            } else {
                if !&self.positions.contains(&p.right()) {
                    right_sides.insert(p.clone());
                }
            }
        }

        let mut tops: HashMap<usize, Vec<GridPosition>> = HashMap::new();
        for p in top_sides {
            if let Some(existing) = tops.get_mut(&p.row) {
                existing.push(p.clone());
            } else {
                tops.insert(p.row, vec![p.clone()]);
            }
        }
        let mut bottoms: HashMap<usize, Vec<GridPosition>> = HashMap::new();
        for p in bottom_sides {
            if let Some(existing) = bottoms.get_mut(&p.row) {
                existing.push(p.clone());
            } else {
                bottoms.insert(p.row, vec![p.clone()]);
            }
        }

        let mut hs = 0;
        let mut h_gaps = 0;
        for (k, mut v) in tops {
            v.sort_by(|a,b| a.col.cmp(&b.col));
            hs += 1;
            println!("H{} -> {:?}", k, v);
            for (left, right) in v.iter().tuple_windows() {
                if &left.right() != right {
                    h_gaps += 1;
                }
            }
        }
        for (k, mut v) in bottoms {
            v.sort_by(|a,b| a.col.cmp(&b.col));
            hs += 1;
            println!("H{} -> {:?}", k, v);
            for (left, right) in v.iter().tuple_windows() {
                if &left.right() != right {
                    h_gaps += 1;
                }
            }
        }

        let mut lefts: HashMap<usize, Vec<GridPosition>> = HashMap::new();
        for p in left_sides {
            if let Some(existing) = lefts.get_mut(&p.col) {
                existing.push(p.clone());
            } else {
                lefts.insert(p.col, vec![p.clone()]);
            }
        }
        let mut rights: HashMap<usize, Vec<GridPosition>> = HashMap::new();
        for p in right_sides {
            if let Some(existing) = rights.get_mut(&p.col) {
                existing.push(p.clone());
            } else {
                rights.insert(p.col, vec![p.clone()]);
            }
        }
        
        let mut vs = 0;
        let mut v_gaps = 0;
        for (k, mut v) in lefts {
            v.sort_by(|a,b| a.row.cmp(&b.row));
            vs += 1;
            println!("V{} -> {:?}", k, v);
            for (top, bottom) in v.iter().tuple_windows() {
                if &top.down() != bottom {
                    v_gaps += 1;
                }
            }
        }
        for (k, mut v) in rights {
            v.sort_by(|a,b| a.row.cmp(&b.row));
            vs += 1;
            println!("V{} -> {:?}", k, v);
            for (top, bottom) in v.iter().tuple_windows() {
                if &top.down() != bottom {
                    v_gaps += 1;
                }
            }
        }

        println!("hs: {} h_gaps: {} vs: {} v_gaps: {}", hs, h_gaps, vs, v_gaps);
        hs + h_gaps + vs + v_gaps
    }

    fn area(&self) -> usize {
        self.positions.len()
    }

    fn price(&self, g: &Grid<Plant>) -> usize {
        self.area() * self.perimeter(g)
    }

    fn discount_price(&self, g: &Grid<Plant>) -> usize {
        self.area() * self.sides(g)
    }
}

impl FromChar for Plant {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        Ok(Plant(c))
    }
}

fn flood_fill(
    plant_type: &Plant,
    pos: &GridPosition,
    grid: &Grid<Plant>,
    seen: &mut HashSet<GridPosition>,
) {
    if seen.contains(pos) {
        return;
    }

    let plant = grid.at(pos);
    if plant != plant_type {
        return;
    }

    seen.insert(pos.clone());

    if pos.col > 0 {
        let left_pos = pos.left();
        flood_fill(plant_type, &left_pos, grid, seen);
    }
    if pos.col < grid.width() - 1 {
        let right_pos = pos.right();
        flood_fill(plant_type, &right_pos, grid, seen);
    }
    if pos.row > 0 {
        let up_pos = pos.up();
        flood_fill(plant_type, &up_pos, grid, seen);
    }
    if pos.row < grid.height() - 1 {
        let up_pos = pos.down();
        flood_fill(plant_type, &up_pos, grid, seen);
    }
}

#[cfg(test)]
mod tests {    
    use itertools::Itertools;

    use crate::*;

    fn print(g: Garden) {
        println!("Garden Report");
        for plot in g.1 {
            println!("{:?} - A {}, P {}", plot.plant, plot.area(), plot.perimeter(&g.0));
            for p in plot.positions {
                println!("{},{}", p.col, p.row);
            }
            println!("")
        }
    }

    fn print_2(g: Garden) {
        println!("Garden Report");
        for plot in g.1 {
            println!("{:?} - A {}, S {}", plot.plant, plot.area(), plot.sides(&g.0));
            for p in plot.positions {
                println!("{},{}", p.col, p.row);
            }
            println!("")
        }
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        // print(test_input.parse().unwrap());
        assert_eq!(1930, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part1(test_input));
    }

    #[test]
    fn input_test_pt2_e() {
        let test_input = include_str!("input.part2.test.txt");
        print_2(test_input.parse().unwrap());
        assert_eq!(236, part2(test_input));
    }

    #[test]
    fn input_test_pt2() {
        let test_input = include_str!("input.test.txt");
        print_2(test_input.parse().unwrap());
        assert_eq!(1206, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(0, part2(test_input));
    }
}
