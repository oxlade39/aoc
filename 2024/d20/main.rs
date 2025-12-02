use core::str;
use std::{i64, time::Instant, usize};

use aoclib::{
    grid::{FromChar, Grid, GridPosition},
    timing,
};
use hashbrown::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    find_all_shortcuts(txt, 100)
}

fn part2(txt: &str) -> usize {
    let savings = list_savings(txt, 100, 20);
    savings.values().sum()
}

fn list_savings(txt: &str, min: usize, picoseconds: usize) -> HashMap<usize, usize> {
    let g: Grid<Tile> = txt.parse().unwrap();
    let normal_path = path(&g);

    let mut savings: HashMap<usize, usize> = HashMap::new();
    for i in 0..normal_path.len() {
        for j in i..normal_path.len() {
            let position_i = normal_path[i];
            let position_j = normal_path[j];
            let manhatten_distance =
                position_i.col.abs_diff(position_j.col) + position_i.row.abs_diff(position_j.row);
            if manhatten_distance <= picoseconds {
                let saving = (j as i64) - (i as i64) - (manhatten_distance as i64);
                if saving >= min as i64 {
                    let saving = saving as usize;
                    savings.insert(saving, savings.get(&saving).unwrap_or(&0) + 1);
                }
            }
        }
    }
    savings
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
    shortcuts.iter().filter(|s| s.saving >= gte).count()
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
                    let left_left_path_index = position_index
                        .get(&left_left)
                        .expect("space must be on path");
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
                    let right_right_path_index = position_index
                        .get(&right_right)
                        .expect("space must be on path");
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
                    let up_up_path_index =
                        position_index.get(&up_up).expect("space must be on path");
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
                    let down_down_path_index = position_index
                        .get(&down_down)
                        .expect("space must be on path");
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
    path.push(end(&g));
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
    use itertools::Itertools;

    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(44, find_all_shortcuts(test_input, 1));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1502, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        let savings = list_savings(test_input, 50, 20);
        let total: usize = savings.values().sum();
        assert_eq!(
            32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3,
            total
        );
    }

    #[allow(dead_code)]
    fn print_savings(savings: HashMap<usize, usize>) {
        for (saving, number) in savings.iter().sorted_by_key(|(saving, _number)| *saving) {
            println!(
                "There are {} cheats that save {} picoseconds",
                number, saving
            );
        }
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        // 1045 too low
        // 22951 too low
        // 2789483 too high
        let pt2 = part2(test_input);
        assert!(pt2 > 1045, "{}", pt2);
        assert!(pt2 > 22951, "{}", pt2);
        assert!(pt2 < 2789483, "{}", pt2);
        assert_eq!(1028136, pt2);
    }
}
