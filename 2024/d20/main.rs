use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{
    grid::{FromChar, Grid, GridPosition},
    input, timing,
};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.test.txt");
    let now = Instant::now();
    // println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    find_all_shortcuts(txt, 100)
}

fn part2(txt: &str) -> usize {
    count_all_shortcuts(txt, 100).len()
}

fn find_all_shortcuts(txt: &str, gte: usize) -> usize {
    let g: Grid<Tile> = txt.parse().unwrap();
    let normal_path = path(&g);
    let e = end(&g);
    let mut position_index: HashMap<GridPosition, usize> = normal_path    
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, value)| (value, index + 1))
        .collect();
    position_index.insert(e, normal_path.len() + 1);

    let shortcuts = build_shortcuts(&g, &normal_path, &position_index);

    for (g, s) in &shortcuts.iter()
        .sorted_by_key(|s| s.saving)
        .group_by(|s| s.saving) {
        println!("There are {} cheats that save {} picoseconds", s.count(), g);
    }

    shortcuts.iter().filter(|s| s.saving >= gte).count()
}

fn count_all_shortcuts(txt: &str, min_saving: usize) -> Vec<Shortcut> {
    let g: Grid<Tile> = txt.parse().unwrap();
    let normal_path = path(&g);
    let e = end(&g);
    let mut position_index: HashMap<GridPosition, usize> = normal_path    
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, value)| (value, index + 1))
        .collect();
    position_index.insert(e, normal_path.len() + 1);

    find_all_within(&g, start(&g), &position_index);
    todo!()
}

fn shortcuts_from(
    start: GridPosition,
    current: GridPosition,
    count: usize,
    max_count: usize,
    g: &Grid<Tile>,
    position_index: &HashMap<GridPosition, usize>,
    min_saving: usize,
    seen: &mut HashSet<(GridPosition, usize)>,
) -> Vec<Shortcut> {

    if count == max_count {
        return vec![];
    }
    let mut all = Vec::new();

    seen.insert((current.clone(), count));
    println!("{:?} {:?}", current, count);

    let tile_at_current = g.at(&current);

    if tile_at_current == &Tile::Space || tile_at_current == &Tile::End {
        let step = *position_index.get(&start).expect("position on path") as i64;
        let current_index = *position_index.get(&current).expect("position on path") as i64;
        let saving = current_index - step - count as i64;

        if saving >= min_saving as i64 {
            all.push(Shortcut { from: start, to: current, wall: current, step_from: step as usize, step_to: current_index as usize, saving: saving as usize });
        }
    } 
    
    if current.col > 0 {
        let next = current.left();
        if !seen.contains(&(next, count + 1)) {
            all.extend(shortcuts_from(start, next, count + 1, max_count, g, position_index, min_saving, seen));
        }
    }
    if current.col < g.width() - 1 {
        let next = current.right();
        if !seen.contains(&(next, count + 1)) {
            all.extend(shortcuts_from(start, next, count + 1, max_count, g, position_index, min_saving, seen));
        }
    }
    if current.row > 0 {
        let next = current.up();
        if !seen.contains(&(next, count + 1)) {
            all.extend(shortcuts_from(start, next, count + 1, max_count, g, position_index, min_saving, seen));
        }
    }
    if current.row < g.height() - 1 {
        let next = current.down();
        if !seen.contains(&(next, count + 1)) {
            all.extend(shortcuts_from(start, next, count + 1, max_count, g, position_index, min_saving, seen));
        }
    }

    // backtracking
    // seen.remove(&(current, count));

    all
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Shortcut {
    from: GridPosition,
    to: GridPosition,
    wall: GridPosition,
    step_from: usize,
    step_to: usize,
    saving: usize,
}

fn build_shortcuts(
    g: &Grid<Tile>,
    normal_path: &Vec<GridPosition>,
    position_index: &HashMap<GridPosition, usize>,
) -> Vec<Shortcut> {
    let mut shortcuts = Vec::new();
    for (step, p) in normal_path.iter().enumerate() {
        let step = step + 1;
        // wall to left
        if p.col > 1 {
            let left = p.left();
            if g.at(&left) == &Tile::Wall {
                let left_left = left.left();
                if g.at(&left_left) == &Tile::Space || g.at(&left_left) == &Tile::End {
                    let left_left_path_index = position_index.get(&left_left).expect("space must be on path");
                    if *left_left_path_index > step {
                        let saving = left_left_path_index - step - 2;
                        if saving > 0 {
                            shortcuts.push(Shortcut { 
                                from: p.clone(), 
                                to: left_left, 
                                wall: left, 
                                step_from: step,
                                step_to: *left_left_path_index, 
                                saving,
                            });
                        }                        
                    }
                }
            }
        }

        // wall to right
        if p.col < g.width() - 2 {
            let right = p.right();
            if g.at(&right) == &Tile::Wall {
                let right_right = right.right();
                if g.at(&right_right) == &Tile::Space || g.at(&right_right) == &Tile::End {
                    let right_right_path_index = position_index.get(&right_right).expect("space must be on path");
                    if *right_right_path_index > step {
                        let saving = right_right_path_index - step - 2;
                        if saving > 0 {
                            shortcuts.push(Shortcut { 
                                from: p.clone(), 
                                to: right_right, 
                                wall: right, 
                                step_from: step,
                                step_to: *right_right_path_index,
                                saving,
                            });
                        }                        
                    }
                }
            }
        }

        // wall above
        if p.row > 1 {
            let up = p.up();
            if g.at(&up) == &Tile::Wall {
                let up_up = up.up();
                if g.at(&up_up) == &Tile::Space || g.at(&up_up) == &Tile::End {
                    let up_up_path_index = position_index.get(&up_up).expect("space must be on path");
                    if *up_up_path_index > step {
                        let saving = up_up_path_index - step - 2;
                        if saving > 0 {
                            shortcuts.push(Shortcut { 
                                from: p.clone(), 
                                to: up_up, 
                                wall: up,
                                step_from: step,
                                step_to: *up_up_path_index,
                                saving,
                            });
                        }                        
                    }
                }
            }
        }

        // wall below
        if p.row < g.height() - 2 {
            let down = p.down();
            if g.at(&down) == &Tile::Wall {
                let down_down = down.down();
                if g.at(&down_down) == &Tile::Space || g.at(&down_down) == &Tile::End {
                    let down_down_path_index = position_index.get(&down_down).expect("space must be on path");
                    if *down_down_path_index > step {
                        let saving = down_down_path_index - step - 2;
                        if saving > 0 {
                            shortcuts.push(Shortcut { 
                                from: p.clone(), 
                                to: down_down, 
                                wall: down, 
                                step_from: step,
                                step_to: *down_down_path_index,
                                saving, 
                            });
                        }                        
                    }
                }
            }
        }
    }
    shortcuts
}

fn path(g: &Grid<Tile>) -> Vec<GridPosition> {
    let mut path: Vec<GridPosition> = Vec::new();

    let mut current = start(&g);
    path.push(current);
    let mut prev = current;
    while let Some(p) = next(&g, &prev, &current) {
        prev = current;
        current = p;
        path.push(p.clone());
    }
    path
}

fn next(g: &Grid<Tile>, prev: &GridPosition, current: &GridPosition) -> Option<GridPosition> {
    if current.col > 0 {
        let left = current.left();
        if &left != prev && g.at(&left) == &Tile::Space {
            return Some(left);
        }
    }
    if current.col < g.width() - 1 {
        let right = current.right();
        if &right != prev && g.at(&right) == &Tile::Space {
            return Some(right);
        }
    }
    if current.row > 0 {
        let up = current.up();
        if &up != prev && g.at(&up) == &Tile::Space {
            return Some(up);
        }
    }
    if current.row < g.height() - 1 {
        let down = current.down();
        if &down != prev && g.at(&down) == &Tile::Space {
            return Some(down);
        }
    }
    None
}

fn start(g: &Grid<Tile>) -> GridPosition {
    let (p, _) = g
        .position_itr()
        .find(|(_p, t)| t == &&Tile::Start)
        .expect("Start");
    p
}

fn end(g: &Grid<Tile>) -> GridPosition {
    let (p, _) = g
        .position_itr()
        .find(|(_p, t)| t == &&Tile::End)
        .expect("End");
    p
}

fn find_all_within(
    g: &Grid<Tile>,
    p: GridPosition,
    path_positions: &HashMap<GridPosition, usize>,
) {
    let mut shortcuts: HashSet<(GridPosition, usize)> = HashSet::new();
    let mut capture = |child: GridPosition, _tile: &Tile, cheat_path_len| {
        let start_point = path_positions.get(&p).expect("start on path");
        if let Some(child_point) = path_positions.get(&child) {
            // println!("{:?} - {:?} - {}", p, child_point, cheat_path_len);
            let saving = (*child_point as i64) - (*start_point as i64) - (cheat_path_len as i64);
            if saving >= 76 {                
                shortcuts.insert((child, saving as usize));
            }            
        }
    };
    g.walk_grid(p, 20, &mut capture);
    for (p, s) in shortcuts {
        println!("{:?}, {:?}", p, s);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Space,
    Start,
    End,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Space),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            other => Err(format!("Bad tile: {:?}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1_print() {
        let test_input = include_str!("input.test.txt");
        let g: Grid<Tile> = test_input.parse().unwrap();
        let normal_path = path(&g);
        let position_index: HashMap<GridPosition, usize> = normal_path    
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, value)| (value, index + 1))
            .collect();

        println!("Normal time is {} picoseconds", normal_path.len() + 1);

        let shortcuts = build_shortcuts(&g, &normal_path, &position_index);
        
        for shortcut in shortcuts {
            println!("Saving: {} of {} to {}", shortcut.saving, shortcut.step_from, shortcut.step_to);
            for row in 0..g.height() {
                for col in 0..g.width() {
                    let p = GridPosition::new(col, row);
                    if p == shortcut.wall {
                        print!("1");
                    } else if p == shortcut.to {
                        print!("2");
                    } else {
                        match g.at(&p) {
                            Tile::Wall => print!("#"),
                            Tile::Space => print!("."),
                            Tile::Start => print!("S"),
                            Tile::End => print!("E"),
                        }
                    }
                }
                println!("");
            }
            println!("");
            println!("");
        }
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(44, find_all_shortcuts(test_input, 1));
    }

    #[test]
    fn input_pt1() {
        // 1500 too low
        let test_input = include_str!("input.txt");
        assert_eq!(1502, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(3, count_all_shortcuts(test_input, 76).len());
    }

    #[test]
    fn test_input_pt2_start_end() {
        let test_input = include_str!("input.test.txt");
        let g: Grid<Tile> = test_input.parse().unwrap();
        let normal_path = path(&g);
        let e = end(&g);
        let mut position_index: HashMap<GridPosition, usize> = normal_path    
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, value)| (value, index + 1))
            .collect();
        position_index.insert(e, normal_path.len() + 1);

        find_all_within(&g, start(&g), &position_index);

        // let s = shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 76, &mut HashSet::new());
        // let uniq: HashSet<_> = HashSet::from_iter(s.iter());
        // for s1 in &uniq {
        //     println!("{:?}", s1);
        // }
        // assert_eq!(3, uniq.len());

        // let s = shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 74, &mut HashSet::new());
        // let uniq: HashSet<_> = HashSet::from_iter(s.iter());        
        // assert_eq!(4 + 3, uniq.len());

        // let s = shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 72, &mut HashSet::new());
        // let uniq: HashSet<_> = HashSet::from_iter(s.iter());        
        // assert_eq!(22 + 4 + 3, uniq.len());

        // assert_eq!(4 + 3, shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 74, &mut HashSet::new()));
        // assert_eq!(22 + 4 + 3, shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 72, &mut HashSet::new()));
        // assert_eq!(3 + 4 + 22, shortcuts_from(start(&g), start(&g), 0, 20, &g, &position_index, 72, &mut HashSet::new()));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        // 1045 too low
        // 22951 too low
        // 2789483 too high
        // 1862449
        assert_eq!(0, part2(test_input));
    }
}
