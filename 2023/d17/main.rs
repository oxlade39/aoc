use std::time::Instant;

use aoclib::{
    input::{Grid, GridPosition},
    shortest_path::{self, Heuristic, ManhattenDistanceTo},
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    solve(txt, 1, 3)
}

fn part2(txt: &str) -> usize {
    solve(txt, 4, 10)
}

fn solve(txt: &str, min: usize, max: usize) -> usize {
    let g: Grid<usize> = txt.parse().unwrap();
    let lf: LavaFall = LavaFall {
        map: g,
        min,
        max,
    };
    let initial_state = State {
        grid_pos: GridPosition::new(0, 0),
        direction: Direction::Right,
        direction_count: 1,
    };
    let end_pos = GridPosition::new(lf.map.width() - 1, lf.map.height() - 1);
    let end_state = |es: &State| es.grid_pos == end_pos;
    let result = shortest_path::astar(&lf, &lf, &end_pos, initial_state, end_state).unwrap();

    for row in 0..lf.map.height() {
        for col in 0..lf.map.width() {
            match result
                .path
                .iter()
                .find(|(s, _)| s.grid_pos == GridPosition::new(col, row))
            {
                Some((s, _)) => match s.direction {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                },
                None => {
                    print!(".")
                }
            }
        }
        println!("");
    }

    for (p, c) in &result.path {
        println!("{:?} = {}", p.grid_pos, c);
    }

    result.total_cost
    // result.path.into_iter().map(|(_, cost)| cost).sum()
}

struct LavaFall {
    map: Grid<usize>,
    min: usize,
    max: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    grid_pos: GridPosition,
    direction: Direction,
    direction_count: usize,
}

impl State {
    fn apply(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Self {
                grid_pos: self.grid_pos.up(),
                direction: Direction::Up,
                direction_count: if self.direction == Direction::Up {
                    self.direction_count + 1
                } else {
                    1
                },
            },
            Direction::Down => Self {
                grid_pos: self.grid_pos.down(),
                direction: Direction::Down,
                direction_count: if self.direction == Direction::Down {
                    self.direction_count + 1
                } else {
                    1
                },
            },
            Direction::Left => Self {
                grid_pos: self.grid_pos.left(),
                direction: Direction::Left,
                direction_count: if self.direction == Direction::Left {
                    self.direction_count + 1
                } else {
                    1
                },
            },
            Direction::Right => Self {
                grid_pos: self.grid_pos.right(),
                direction: Direction::Right,
                direction_count: if self.direction == Direction::Right {
                    self.direction_count + 1
                } else {
                    1
                },
            },
        }
    }
}

impl aoclib::shortest_path::Neighbours<State> for LavaFall {
    fn neighbours(&self, state: &State) -> Vec<State> {
        let width = self.map.width();
        let height = self.map.height();

        let current_col = state.grid_pos.col;
        let current_row = state.grid_pos.row;

        let mut next = Vec::new();

        if state.direction_count == self.max {
            match state.direction {
                Direction::Up => {
                    if current_col > 0 {
                        next.push(state.apply(Direction::Left));
                    }
                    if current_col < width - 1 {
                        next.push(state.apply(Direction::Right));
                    }
                }
                Direction::Down => {
                    if current_col > 0 {
                        next.push(state.apply(Direction::Left));
                    }
                    if current_col < width - 1 {
                        next.push(state.apply(Direction::Right));
                    }
                }
                Direction::Left => {
                    if current_row < height - 1 {
                        next.push(state.apply(Direction::Down));
                    }
                    if current_row > 0 {
                        next.push(state.apply(Direction::Up));
                    }
                }
                Direction::Right => {
                    if current_row < height - 1 {
                        next.push(state.apply(Direction::Down));
                    }
                    if current_row > 0 {
                        next.push(state.apply(Direction::Up));
                    }
                }
            }
        } else {
            match state.direction {
                Direction::Up => {
                    if current_row > 0 {
                        next.push(state.apply(Direction::Up));
                    }
                    if state.direction_count >= self.min && current_col > 0 {
                        next.push(state.apply(Direction::Left));
                    }
                    if state.direction_count >= self.min && current_col < width - 1 {
                        next.push(state.apply(Direction::Right));
                    }
                }
                Direction::Down => {
                    if state.direction_count >= self.min && current_col > 0 {
                        next.push(state.apply(Direction::Left));
                    }
                    if state.direction_count >= self.min && current_col < width - 1 {
                        next.push(state.apply(Direction::Right));
                    }
                    if current_row < height - 1 {
                        next.push(state.apply(Direction::Down));
                    }
                }
                Direction::Left => {
                    if current_col > 0 {
                        next.push(state.apply(Direction::Left));
                    }
                    if state.direction_count >= self.min && current_row < height - 1 {
                        next.push(state.apply(Direction::Down));
                    }
                    if state.direction_count >= self.min && current_row > 0 {
                        next.push(state.apply(Direction::Up));
                    }
                }
                Direction::Right => {
                    if state.direction_count >= self.min && current_row < height - 1 {
                        next.push(state.apply(Direction::Down));
                    }
                    if state.direction_count >= self.min && current_row > 0 {
                        next.push(state.apply(Direction::Up));
                    }
                    if current_col < width - 1 {
                        next.push(state.apply(Direction::Right));
                    }
                }
            }
        }
        next
    }
}

impl Heuristic<State, usize> for GridPosition {
    fn predict(&self, from: &State) -> usize {
        ManhattenDistanceTo(*self).predict(&from.grid_pos)
    }
}

impl shortest_path::Cost<State, usize> for LavaFall {
    fn measure(&self, to: &State) -> usize {
        *self.map.at(&to.grid_pos)
    }
}

#[cfg(test)]
mod tests {
    use aoclib::shortest_path::Cost;

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(102, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_regression() {
        // fails :( -> should be 1023
        assert_eq!(1024, part1(include_str!("input.txt")));
        // but pt2 works
        assert_eq!(1165, part2(include_str!("input.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(94, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_cost() {
        let txt = include_str!("input.test.txt");
        let g: Grid<usize> = txt.parse().unwrap();
        let lf = LavaFall {
            map: g,
            min: 1,
            max: 3,
        };
        let cost = lf.measure(&State {
            grid_pos: GridPosition::new(0, 0),
            direction: Direction::Right,
            direction_count: 1,
        });
        assert_eq!(2, cost);
    }
}
