use std::{
    collections::HashSet,
    fmt::{Display, Write},
    time::Instant,
};

use aoclib::{
    cartesian::{Point, Transform},
    grid::{Flip, FromChar, Grid, GridPosition},
};
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let g: Grid<_> = txt.parse().unwrap();
    let map = Map(g.flip());
    map.steps(64).len()
}

fn part2(txt: &str) -> i64 {
    // puzzle target steps
    let target = 26501365;

    let g: Grid<Position> = txt.parse().unwrap();
    let map = Map(g.flip());

    let s = map.start_pos().unwrap();
    let distance_to_side = s.x;

    let samples: Vec<_> = (0..4).map(|i| grids_to_count(&map, i + 1)).collect();
    let deltas: Vec<_> = samples
        .iter()
        .tuple_windows()
        .map(|(left, right)| right - left)
        .collect();
    let second_deltas: Vec<_> = deltas
        .iter()
        .tuple_windows()
        .map(|(left, right)| right - left)
        .collect();

    if second_deltas[0] != second_deltas[1] {
        panic!("pattern non repeating at second level");
    }

    let increment = second_deltas[0] as i64;

    if (target - distance_to_side) % map.0.width() as i64 != 0 {
        panic!("target was not an exact number of grids, interpolation logic won't work");
    }

    let target_step = (target - distance_to_side) / map.0.width() as i64;

    let mut step = samples.len() as i64 + 1;
    let mut step_increment = *deltas.last().unwrap() as i64;
    let mut value = *samples.last().unwrap() as i64;
    loop {
        step += 1;
        step_increment += increment;
        value += step_increment;

        // println!("step {} = {}", step, value);

        // have some off by ones somewhere
        if step == target_step + 2 {
            break;
        }
    }

    value
}

#[allow(dead_code)]
fn grids_to_count(map: &Map, grid: usize) -> usize {
    let s = map.start_pos().unwrap();
    let distance_to_side = s.x;
    let d = distance_to_side + (map.0.width() as i64 * (grid as i64 - 1));
    map.steps(d as usize).len()
}

#[derive(Debug, Clone)]
struct Map(Grid<Position>);

fn mod_dimension(max: usize, dimension: i64) -> i64 {
    dimension.rem_euclid(max as i64)
}

impl Map {
    fn at(&self, p: &Point) -> &Position {
        let mod_col = mod_dimension(self.0.width(), p.x);
        let mod_row = mod_dimension(self.0.height(), p.y);

        if mod_col < 0
            || mod_row < 0
            || mod_col >= self.0.width() as i64
            || mod_row >= self.0.height() as i64
        {
            panic!(
                "{p:?} height {} modified to {mod_col},{mod_row}",
                self.0.height()
            );
        }

        self.0
            .at(&GridPosition::new(mod_col as usize, mod_row as usize))
    }

    fn start_pos(&self) -> Option<Point> {
        for row in 0..self.0.height() {
            let y = self.0.height() - row;
            for x in 0..self.0.width() {
                let pos = Point::new(x as i64, y as i64);
                if self.at(&pos) == &Position::Start {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn can_walk_to(&self, p: &Point) -> bool {
        match self.at(p) {
            Position::Start => true,
            Position::GardenPlot => true,
            Position::Rock => false,
        }
    }

    fn step(&self, current: HashSet<Point>) -> HashSet<Point> {
        let mut next: HashSet<Point> = HashSet::new();
        let mut position_itr = current.into_iter();

        while let Some(p) = position_itr.next() {
            next.insert(p.transform(&Transform::left()));
            next.insert(p.transform(&Transform::right()));
            next.insert(p.transform(&Transform::up()));
            next.insert(p.transform(&Transform::down()));
        }

        next.into_iter().filter(|p| self.can_walk_to(p)).collect()
    }

    fn steps(&self, num: usize) -> HashSet<Point> {
        let start: HashSet<Point> = self.start_pos().into_iter().collect();

        (0..num)
            .into_iter()
            .fold(start, |accum, _| self.step(accum))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Start,
    GardenPlot,
    Rock,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Start => f.write_char('S'),
            Position::GardenPlot => f.write_char('.'),
            Position::Rock => f.write_char('#'),
        }
    }
}

impl FromChar for Position {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'S' => Ok(Position::Start),
            '.' => Ok(Position::GardenPlot),
            '#' => Ok(Position::Rock),
            other => Err(format!("bad char: {}", other)),
        }
    }
}

impl Flip for Position {
    fn flip(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_p1() {
        let txt = include_str!("input.test.txt");
        let map = Map(txt.parse().unwrap());
        assert_eq!(16, map.steps(6).len());
    }

    #[test]
    fn test_part2_infinite_grid_example() {
        let origin = include_str!("input.test.txt");
        let g_orig: Grid<Position> = origin.parse().unwrap();
        let m_orig = Map(g_orig.flip());

        let origin_expand = include_str!("input.test.expanded.txt");
        let g_orig_expand: Grid<Position> = origin_expand.parse().unwrap();
        let m_orig_expand = Map(g_orig_expand.flip());

        let orig_start = m_orig.start_pos().unwrap();
        let expand_start = m_orig_expand.start_pos().unwrap();

        for y in 0..m_orig_expand.0.height() {
            for x_delta in 0..m_orig_expand.0.width() {
                let orig = orig_start.transform(&(x_delta as i64, y as i64).into());
                let expand = expand_start.transform(&(x_delta as i64, y as i64).into());

                assert_eq!(
                    m_orig.can_walk_to(&orig),
                    m_orig_expand.can_walk_to(&expand),
                    "{}",
                    x_delta
                )
            }
        }
    }

    #[test]
    fn test_mod_dimension_wraps_around() {
        let origin = include_str!("input.test.txt");
        let g_orig: Grid<Position> = origin.parse().unwrap();
        let m_orig = Map(g_orig.flip());

        let width = m_orig.0.width();

        let result = mod_dimension(width, -1);
        assert_eq!(m_orig.0.width() as i64 - 1, result);

        let result = mod_dimension(width, -11);
        assert_eq!(0, result);

        let result = mod_dimension(width, -12);
        assert_eq!(m_orig.0.width() as i64 - 1, result);
    }

    #[test]
    fn test_part2_expanded_error() {
        let origin = include_str!("input.test.txt");
        let g_orig: Grid<Position> = origin.parse().unwrap();
        let m_orig = Map(g_orig.flip());
        let orig_start = m_orig.start_pos().unwrap();

        let expanded_steps = m_orig.steps(7);
        print_steps(&m_orig, &expanded_steps);

        assert_eq!(true, expanded_steps.contains(&Point::new(0, 5)));

        println!(
            "{:?}",
            m_orig.at(&Point::new(0, 5).transform(&Transform::left()))
        );
        assert_eq!(
            true,
            m_orig.can_walk_to(&Point::new(0, 5).transform(&Transform::left()))
        );

        println!();
        println!();

        let expanded_steps = m_orig.steps(8);
        print_steps(&m_orig, &expanded_steps);
    }

    #[test]
    fn test_part2_expanded() {
        let origin = include_str!("input.test.txt");
        let g_orig: Grid<Position> = origin.parse().unwrap();
        let m_orig = Map(g_orig.flip());
        let orig_start = m_orig.start_pos().unwrap();

        let origin_expand = include_str!("input.test.expanded.txt");
        let g_orig_expand: Grid<Position> = origin_expand.parse().unwrap();
        let m_orig_expand = Map(g_orig_expand.flip());
        let expand_start = m_orig_expand.start_pos().unwrap();

        println!("width: {}", m_orig.0.width());

        for i in 0..10 {
            let expanded_steps = m_orig_expand.steps(i);
            let orig_steps = m_orig.steps(i);

            // if expanded_steps.len() != orig_steps.len() {
            println!("expanded S = {:?}", m_orig_expand.start_pos().unwrap());
            for y in 9..25 {
                print!("{y}\t");
                for x in 6..30 {
                    let p = (x as i64, y as i64).into();
                    if expand_start == p {
                        print!("S");
                    } else if expanded_steps.contains(&p) {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            println!("origin S = {:?}", m_orig.start_pos().unwrap());
            for y in -2..14 {
                print!("{y}\t");
                for x in -5..19 {
                    let p = (x as i64, y as i64).into();
                    if orig_start == p {
                        print!("S");
                    } else if orig_steps.contains(&p) {
                        print!("O");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }
            // }
            println!("");
            println!("");
            println!("");

            assert_eq!(orig_steps.len(), expanded_steps.len(), "failed at {}", i);
        }
    }

    fn print_steps(m: &Map, points: &HashSet<Point>) {
        let start = m.start_pos().unwrap();

        let min_y = points.iter().map(|p| p.y).min().unwrap() - 5;
        let max_y = points.iter().map(|p| p.y).max().unwrap() + 5;

        let min_x = points.iter().map(|p| p.x).min().unwrap() - 5;
        let max_x = points.iter().map(|p| p.x).max().unwrap() + 5;

        println!("steps {}", points.len());
        for y in min_y..max_y {
            print!("{y}\t");
            for x in min_x..max_x {
                let p = (x as i64, y as i64).into();
                if start == p {
                    print!("S");
                } else if points.contains(&p) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    #[test]
    fn test_part_2() {
        let txt = include_str!("input.test.txt");
        let g: Grid<_> = txt.parse().unwrap();
        let m = Map(g.flip());

        let expanded_txt = include_str!("input.test.expanded.txt");
        let expanded_g: Grid<_> = expanded_txt.parse().unwrap();
        let expanded_m = Map(expanded_g.flip());

        let steps = [
            (6, 16),
            (10, 50),
            (50, 1594),
            // (100, 6536),
            // (500, 167004),
            // (1000, 668697),
            // (5000, 16733044),
        ];

        print_steps(&expanded_m, &expanded_m.steps(10));
        println!();
        println!();
        print_steps(&m, &m.steps(10));

        for (i, expected) in steps {
            assert_eq!(expected, m.steps(i).len());
        }
    }

    #[test]
    fn test_example_pt1_steps() {
        let txt = include_str!("input.test.txt");
        let map = Map(txt.parse().unwrap());

        let positions = map.start_pos().into_iter().collect();
        let next = map.step(positions);
        assert_eq!(2, next.len());

        let next = map.step(next);
        assert_eq!(4, next.len());
    }

    #[test]
    fn test_infinite_width_height() {
        let txt = include_str!("input.test.txt");
        let map = Map(txt.parse().unwrap());

        for i in 0..map.0.width() {
            let off_grid_col = 0 - i as i64;
            let row = 0;
            assert_eq!(
                map.at(&Point::new(off_grid_col, row)),
                map.at(&Point::new(map.0.width() as i64 - i as i64, row))
            )
        }
    }
}
