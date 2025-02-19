use core::fmt;
use std::{array, i64, str::FromStr, time::Instant, usize};

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
    let input: Input = txt.parse().unwrap();
    let mut chain: RobotChain<2> = RobotChain::default();
    input
        .0
        .into_iter()
        .map(|item| item.complexity(&mut chain))
        .sum()
}

fn part2(txt: &str) -> usize {
    let input: Input = txt.parse().unwrap();
    let mut chain: RobotChain<25> = RobotChain::default();
    input
        .0
        .into_iter()
        .map(|item| item.complexity(&mut chain))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NumericKeypadTile {
    Number(u8),
    A,
    Blank,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DirectionalKeypadTile {
    Up,
    Down,
    Left,
    Right,
    A,
    Blank,
}

#[derive(Debug)]
struct NumericSequence([NumericKeypadTile; 4]);

struct Input(Vec<NumericSequence>);

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g: Grid<NumericKeypadTile> = s.parse()?;
        let items = g
            .rows
            .into_iter()
            .map(|row| NumericSequence(row.try_into().expect("exactly 4 items")))
            .collect();
        Ok(Input(items))
    }
}

impl NumericSequence {
    fn complexity<const N: usize>(&self, chain: &mut RobotChain<N>) -> usize {
        let mut count = 0;
        for code in &self.0 {
            count += chain.press(code.clone());
        }
        let seq_num: usize = format!("{}{}{}", self.0[0], self.0[1], self.0[2])
            .parse()
            .expect("number");
        seq_num * count
    }
}

impl DirectionalKeypadTile {
    fn position(&self) -> GridPosition {
        match self {
            DirectionalKeypadTile::Blank => panic!("'Blank' has no position"),
            DirectionalKeypadTile::Up => GridPosition::new(1, 0),
            DirectionalKeypadTile::A => GridPosition::new(2, 0),
            DirectionalKeypadTile::Left => GridPosition::new(0, 1),
            DirectionalKeypadTile::Down => GridPosition::new(1, 1),
            DirectionalKeypadTile::Right => GridPosition::new(2, 1),
        }
    }
}

impl NumericKeypadTile {
    fn position(&self) -> GridPosition {
        match self {
            NumericKeypadTile::Number(7) => GridPosition::new(0, 0),
            NumericKeypadTile::Number(8) => GridPosition::new(1, 0),
            NumericKeypadTile::Number(9) => GridPosition::new(2, 0),
            NumericKeypadTile::Number(4) => GridPosition::new(0, 1),
            NumericKeypadTile::Number(5) => GridPosition::new(1, 1),
            NumericKeypadTile::Number(6) => GridPosition::new(2, 1),
            NumericKeypadTile::Number(1) => GridPosition::new(0, 2),
            NumericKeypadTile::Number(2) => GridPosition::new(1, 2),
            NumericKeypadTile::Number(3) => GridPosition::new(2, 2),
            NumericKeypadTile::Blank => GridPosition::new(0, 3),
            NumericKeypadTile::Number(0) => GridPosition::new(1, 3),
            NumericKeypadTile::A => GridPosition::new(2, 3),
            other => panic!("bad tile: {:?}", other),
        }
    }
}

impl FromChar for NumericKeypadTile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'A' => Ok(NumericKeypadTile::A),
            other => other
                .to_digit(10)
                .ok_or(format!("expected number {:?}", other))
                .map(|n| NumericKeypadTile::Number(n as u8)),
        }
    }
}

impl FromChar for DirectionalKeypadTile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '>' => Ok(DirectionalKeypadTile::Right),
            '<' => Ok(DirectionalKeypadTile::Left),
            '^' => Ok(DirectionalKeypadTile::Up),
            'v' => Ok(DirectionalKeypadTile::Down),
            'A' => Ok(DirectionalKeypadTile::A),
            other => Err(format!("unexpected {:?}", other)),
        }
    }
}

impl fmt::Display for NumericKeypadTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericKeypadTile::Number(n) => n.fmt(f),
            NumericKeypadTile::A => 'A'.fmt(f),
            NumericKeypadTile::Blank => panic!("BLANK"),
        }
    }
}

impl fmt::Display for DirectionalKeypadTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalKeypadTile::Up => '^'.fmt(f),
            DirectionalKeypadTile::Down => 'v'.fmt(f),
            DirectionalKeypadTile::Left => '<'.fmt(f),
            DirectionalKeypadTile::Right => '>'.fmt(f),
            DirectionalKeypadTile::A => 'A'.fmt(f),
            DirectionalKeypadTile::Blank => panic!("BLANK"),
        }
    }
}

#[derive(Clone)]
struct KeypadRobot<T> 
    where T: Clone + FromChar,
{
    grid: Grid<T>,
    position: GridPosition,
    move_queue: Vec<T>,
}

impl Default for KeypadRobot<NumericKeypadTile> {
    fn default() -> Self {
        let grid = Grid {
            rows: vec![
                vec![
                    NumericKeypadTile::Number(7),
                    NumericKeypadTile::Number(8),
                    NumericKeypadTile::Number(9),
                ],
                vec![
                    NumericKeypadTile::Number(4),
                    NumericKeypadTile::Number(5),
                    NumericKeypadTile::Number(6),
                ],
                vec![
                    NumericKeypadTile::Number(1),
                    NumericKeypadTile::Number(2),
                    NumericKeypadTile::Number(3),
                ],
                vec![
                    NumericKeypadTile::Blank,
                    NumericKeypadTile::Number(0),
                    NumericKeypadTile::A,
                ],
            ],
        };
        let position = GridPosition::new(2, 3);
        assert!(grid.at(&position) == &NumericKeypadTile::A);
        Self {
            grid,
            position,
            move_queue: Vec::new(),
        }
    }
}

impl Default for KeypadRobot<DirectionalKeypadTile> {
    fn default() -> Self {
        let grid = Grid {
            rows: vec![
                vec![
                    DirectionalKeypadTile::Blank,
                    DirectionalKeypadTile::Up,
                    DirectionalKeypadTile::A,
                ],
                vec![
                    DirectionalKeypadTile::Left,
                    DirectionalKeypadTile::Down,
                    DirectionalKeypadTile::Right,
                ],
            ],
        };
        let position = GridPosition::new(2, 0);
        assert!(grid.at(&position) == &DirectionalKeypadTile::A);
        Self {
            grid,
            position,
            move_queue: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct RobotChain<const N: usize> {
    directional: [KeypadRobot<DirectionalKeypadTile>; N],
    move_cache: HashMap<(usize, DirectionalKeypadTile, DirectionalKeypadTile), usize>,
    numeric: KeypadRobot<NumericKeypadTile>,
}

impl<const N: usize> Default for RobotChain<N> {
    fn default() -> Self {
        Self {
            directional: array::from_fn(|_| Default::default()),
            move_cache: Default::default(),
            numeric: Default::default(),
        }
    }
}

impl<T> KeypadRobot<T> 
    where T: Clone + FromChar,
{
    fn current(&self) -> &T {
        self.grid.at(&self.position)
    }
}

impl KeypadRobot<DirectionalKeypadTile> {
    fn flush_moves(&mut self) -> &DirectionalKeypadTile {
        for m in &self.move_queue {
            let new_pos = match m {
                DirectionalKeypadTile::Up => self.position.up(),
                DirectionalKeypadTile::Down => self.position.down(),
                DirectionalKeypadTile::Left => self.position.left(),
                DirectionalKeypadTile::Right => self.position.right(),
                DirectionalKeypadTile::A => panic!("'A' in move queue"),
                DirectionalKeypadTile::Blank => panic!("'Blank' in move queue"),
            };
            self.position = new_pos;
            assert!(self.current() != &DirectionalKeypadTile::Blank)
        }
        self.move_queue.clear();
        self.current()
    }
}

#[allow(dead_code)]
fn debug_moves(moves: &Vec<DirectionalKeypadTile>) {
    for m in moves {
        print!("{}", m);
    }
    println!("");
}

#[allow(dead_code)]
fn debug_reverse_moves(txt: &str) {
    let mut chain: RobotChain<2> = RobotChain::default();
    let g: Grid<DirectionalKeypadTile> = txt.parse().unwrap();
    println!("");
    for (_, tile) in g.position_itr() {
        if let Some(out) = chain.move_arm(tile.clone()) {
            println!("{:?}", out);
        }
    }
}

trait MovesBetween<T, M> {
    fn moves_between(from: T, to: T) -> Vec<M>;
}

struct ColumnsFirst;
struct RowsFirst;

impl MovesBetween<DirectionalKeypadTile, DirectionalKeypadTile> for RowsFirst {
    fn moves_between(
        from: DirectionalKeypadTile,
        to: DirectionalKeypadTile,
    ) -> Vec<DirectionalKeypadTile> {
        match (from, to) {
            (DirectionalKeypadTile::A, DirectionalKeypadTile::Left) => vec![
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (DirectionalKeypadTile::Left, DirectionalKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Up,
            ],
            (DirectionalKeypadTile::Up, DirectionalKeypadTile::Left) => {
                vec![DirectionalKeypadTile::Down, DirectionalKeypadTile::Left]
            }
            (DirectionalKeypadTile::Left, DirectionalKeypadTile::Up) => {
                vec![DirectionalKeypadTile::Right, DirectionalKeypadTile::Up]
            }
            (from, to) => {
                let mut moves = Vec::new();
                let my_pos = from.position();
                let other_pos = to.position();
                let col_diff = (other_pos.col as i64) - (my_pos.col as i64);
                let row_diff = (other_pos.row as i64) - (my_pos.row as i64);
                if row_diff > 0 {
                    for _right in 0..row_diff {
                        moves.push(DirectionalKeypadTile::Down);
                    }
                }
                if row_diff < 0 {
                    for _right in 0..row_diff.abs() {
                        moves.push(DirectionalKeypadTile::Up);
                    }
                }

                if col_diff > 0 {
                    for _right in 0..col_diff {
                        moves.push(DirectionalKeypadTile::Right);
                    }
                }
                if col_diff < 0 {
                    for _right in 0..col_diff.abs() {
                        moves.push(DirectionalKeypadTile::Left);
                    }
                }
                moves
            }
        }
    }
}

impl MovesBetween<DirectionalKeypadTile, DirectionalKeypadTile> for ColumnsFirst {
    fn moves_between(
        from: DirectionalKeypadTile,
        to: DirectionalKeypadTile,
    ) -> Vec<DirectionalKeypadTile> {
        match (from, to) {
            (DirectionalKeypadTile::A, DirectionalKeypadTile::Left) => vec![
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (DirectionalKeypadTile::Left, DirectionalKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Up,
            ],
            (DirectionalKeypadTile::Up, DirectionalKeypadTile::Left) => {
                vec![DirectionalKeypadTile::Down, DirectionalKeypadTile::Left]
            }
            (DirectionalKeypadTile::Left, DirectionalKeypadTile::Up) => {
                vec![DirectionalKeypadTile::Right, DirectionalKeypadTile::Up]
            }
            (from, to) => {
                let mut moves = Vec::new();
                let my_pos = from.position();
                let other_pos = to.position();
                let col_diff = (other_pos.col as i64) - (my_pos.col as i64);
                let row_diff = (other_pos.row as i64) - (my_pos.row as i64);

                if col_diff > 0 {
                    for _right in 0..col_diff {
                        moves.push(DirectionalKeypadTile::Right);
                    }
                }
                if col_diff < 0 {
                    for _right in 0..col_diff.abs() {
                        moves.push(DirectionalKeypadTile::Left);
                    }
                }

                if row_diff > 0 {
                    for _right in 0..row_diff {
                        moves.push(DirectionalKeypadTile::Down);
                    }
                }
                if row_diff < 0 {
                    for _right in 0..row_diff.abs() {
                        moves.push(DirectionalKeypadTile::Up);
                    }
                }
                moves
            }
        }
    }
}

impl MovesBetween<NumericKeypadTile, DirectionalKeypadTile> for ColumnsFirst {
    fn moves_between(from: NumericKeypadTile, to: NumericKeypadTile) -> Vec<DirectionalKeypadTile> {
        match (from, to) {
            (NumericKeypadTile::Number(7), NumericKeypadTile::Number(0)) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(7), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(4), NumericKeypadTile::Number(0)) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(4), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(1), NumericKeypadTile::Number(0)) => {
                vec![DirectionalKeypadTile::Right, DirectionalKeypadTile::Down]
            }
            (NumericKeypadTile::Number(1), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(1)) => {
                vec![DirectionalKeypadTile::Up, DirectionalKeypadTile::Left]
            }
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(4)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(7)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(1)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(4)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(7)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (from, to) => {
                let my_pos = from.position();
                let other_pos = to.position();

                let mut moves = Vec::new();

                let col_diff = (other_pos.col as i64) - (my_pos.col as i64);
                let row_diff = (other_pos.row as i64) - (my_pos.row as i64);

                if col_diff > 0 {
                    for _right in 0..col_diff {
                        moves.push(DirectionalKeypadTile::Right);
                    }
                }
                if col_diff < 0 {
                    for _right in 0..col_diff.abs() {
                        moves.push(DirectionalKeypadTile::Left);
                    }
                }
                if row_diff > 0 {
                    for _right in 0..row_diff {
                        moves.push(DirectionalKeypadTile::Down);
                    }
                }
                if row_diff < 0 {
                    for _right in 0..row_diff.abs() {
                        moves.push(DirectionalKeypadTile::Up);
                    }
                }
                moves
            }
        }
    }
}

impl MovesBetween<NumericKeypadTile, DirectionalKeypadTile> for RowsFirst {
    fn moves_between(from: NumericKeypadTile, to: NumericKeypadTile) -> Vec<DirectionalKeypadTile> {
        match (from, to) {
            (NumericKeypadTile::Number(7), NumericKeypadTile::Number(0)) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(7), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(4), NumericKeypadTile::Number(0)) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(4), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(1), NumericKeypadTile::Number(0)) => {
                vec![DirectionalKeypadTile::Right, DirectionalKeypadTile::Down]
            }
            (NumericKeypadTile::Number(1), NumericKeypadTile::A) => vec![
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Right,
                DirectionalKeypadTile::Down,
            ],
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(1)) => {
                vec![DirectionalKeypadTile::Up, DirectionalKeypadTile::Left]
            }
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(4)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::Number(0), NumericKeypadTile::Number(7)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(1)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(4)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (NumericKeypadTile::A, NumericKeypadTile::Number(7)) => vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Left,
                DirectionalKeypadTile::Left,
            ],
            (from, to) => {
                let my_pos = from.position();
                let other_pos = to.position();

                let mut moves = Vec::new();

                let col_diff = (other_pos.col as i64) - (my_pos.col as i64);
                let row_diff = (other_pos.row as i64) - (my_pos.row as i64);
                if row_diff > 0 {
                    for _right in 0..row_diff {
                        moves.push(DirectionalKeypadTile::Down);
                    }
                }
                if row_diff < 0 {
                    for _right in 0..row_diff.abs() {
                        moves.push(DirectionalKeypadTile::Up);
                    }
                }

                if col_diff > 0 {
                    for _right in 0..col_diff {
                        moves.push(DirectionalKeypadTile::Right);
                    }
                }
                if col_diff < 0 {
                    for _right in 0..col_diff.abs() {
                        moves.push(DirectionalKeypadTile::Left);
                    }
                }
                moves
            }
        }
    }
}

impl<const R: usize> RobotChain<R> {
    fn press(&mut self, to: NumericKeypadTile) -> usize {
        // for each numeric keypad move
        // calculate the moves required by the inner robot
        // for each press of the inner robot
        // calculate the presses required for the outer robot

        let mut count = 0;

        let current_numeric_pos = self.numeric.current().clone();

        // try cols first moves
        let cols_first_count = {
            let mut cols_first_moves =
                ColumnsFirst::moves_between(current_numeric_pos.clone(), to.clone());
            cols_first_moves.push(DirectionalKeypadTile::A);
            self.down(R, cols_first_moves.clone())
        };

        // then rows first moves
        let rows_first_count = {
            let mut rows_first_moves = RowsFirst::moves_between(current_numeric_pos.clone(), to.clone());
            rows_first_moves.push(DirectionalKeypadTile::A);
            self.down(R, rows_first_moves)
        };

        // take the min from both
        count += cols_first_count.min(rows_first_count);

        self.numeric.position = to.position();
        assert!(self.numeric.current() != &NumericKeypadTile::Blank);

        count
    }

    fn down(&mut self, depth: usize, moves: Vec<DirectionalKeypadTile>) -> usize {
        if depth > 0 {
            let mut total = 0;
            for next_level_move in moves {
                let robot = &self.directional[depth - 1];
                let robot_current_pos = robot.current().clone();

                match self.move_cache.get(&(
                    depth,
                    robot_current_pos.clone(),
                    next_level_move.clone(),
                )) { Some(cached) => {
                    // seen move before
                    total += cached;
                } _ => {
                    // make move and store

                    // try cols first moves
                    let cols_first_count = {
                        let mut cols_first_moves = ColumnsFirst::moves_between(
                            robot_current_pos.clone(),
                            next_level_move.clone(),
                        );
                        cols_first_moves.push(DirectionalKeypadTile::A);
                        self.down(depth - 1, cols_first_moves.clone())
                    };

                    // then rows first moves
                    let rows_first_count = {
                        let mut rows_first_moves =
                            RowsFirst::moves_between(robot_current_pos.clone(), next_level_move.clone());
                        rows_first_moves.push(DirectionalKeypadTile::A);
                        self.down(depth - 1, rows_first_moves)
                    };

                    // take the min from both
                    let child_count = cols_first_count.min(rows_first_count);

                    total += child_count;
                    self.move_cache.insert(
                        (depth, robot_current_pos.clone(), next_level_move.clone()),
                        child_count,
                    );
                }}
                self.directional[depth - 1].position = next_level_move.position();
                assert!(self.directional[depth - 1].current() != &DirectionalKeypadTile::Blank);
            }
            total
        } else {
            // we're at the bottom - these are the moves we want
            moves.len()
        }
    }

    fn move_arm(&mut self, dir: DirectionalKeypadTile) -> Option<NumericKeypadTile> {
        match dir {
            DirectionalKeypadTile::A => {
                // println!("directional[0] queue: {:?}", self.directional[0].move_queue);
                let m = self.directional[0].flush_moves().clone();
                match m {
                    DirectionalKeypadTile::A => {
                        // println!("directional[1] queue: {:?}", self.directional[0].move_queue);
                        let m1 = self.directional[1].flush_moves().clone();
                        match m1 {
                            DirectionalKeypadTile::A => {
                                // apply final layer
                                let num = self.numeric.grid.at(&self.numeric.position).clone();
                                // println!("numberic queue: {:?}", self.numeric.move_queue);
                                self.numeric.move_queue.clear();
                                return Some(num);
                            }
                            dir => {
                                let moved = match dir {
                                    DirectionalKeypadTile::Up => self.numeric.position.up(),
                                    DirectionalKeypadTile::Down => self.numeric.position.down(),
                                    DirectionalKeypadTile::Left => self.numeric.position.left(),
                                    DirectionalKeypadTile::Right => self.numeric.position.right(),
                                    DirectionalKeypadTile::A => panic!("'A' impossible"),
                                    DirectionalKeypadTile::Blank => panic!("'Blank'"),
                                };
                                self.numeric.position = moved;
                                assert!(self.numeric.current() != &NumericKeypadTile::Blank);
                            }
                        }
                    }
                    other => {
                        self.directional[1].move_queue.push(other);
                    }
                }
            }
            movement => {
                self.directional[0].move_queue.push(movement);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_moves() {
        let from = NumericKeypadTile::Number(0);
        assert_eq!(
            RowsFirst::moves_between(from.clone(), NumericKeypadTile::Number(1)),
            vec![DirectionalKeypadTile::Up, DirectionalKeypadTile::Left,]
        );
        assert_eq!(
            RowsFirst::moves_between(from.clone(), NumericKeypadTile::Number(2)),
            vec![DirectionalKeypadTile::Up,]
        );
        assert_eq!(
            RowsFirst::moves_between(from.clone(), NumericKeypadTile::Number(3)),
            vec![DirectionalKeypadTile::Up, DirectionalKeypadTile::Right,]
        );
        assert_eq!(
            RowsFirst::moves_between(from.clone(), NumericKeypadTile::Number(9)),
            vec![
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Up,
                DirectionalKeypadTile::Right,
            ]
        );
    }

    #[test]
    fn test_complexity() {
        let mut chain: RobotChain<2> = RobotChain::default();
        let example: Input = "029A".parse().unwrap();
        let first = example.0.into_iter().next().unwrap();
        assert_eq!(68 * 29, first.complexity(&mut chain));
    }

    #[test]
    fn test_numeric_press_379a() {
        let mut chain: RobotChain<2> = RobotChain::default();
        let mut count = 0;
        count += chain.press(NumericKeypadTile::Number(3));
        count += chain.press(NumericKeypadTile::Number(7));
        count += chain.press(NumericKeypadTile::Number(9));
        count += chain.press(NumericKeypadTile::A);
        assert_eq!(64, count);
        // debug_moves(&all);
    }

    #[test]
    fn test_input_pt1() {
        let mut chain: RobotChain<2> = RobotChain::default();
        let test_input = include_str!("input.test.txt");
        let example: Input = test_input.parse().unwrap();
        let results: Vec<_> = example.0.into_iter().map(|n| n.complexity(&mut chain)).collect();
        // 68 * 29, 60 * 980, 68 * 179, 64 * 456, and 64 * 379
        assert_eq!(results, vec![
            68 * 29, 
            60 * 980, 
            68 * 179, 
            64 * 456, 
            64 * 379
        ]);
        assert_eq!(126384, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        let pt1 = part1(test_input);
        assert_eq!(184718, pt1);
    }

    // #[test]
    // fn part1_reverse() {
    //     debug_reverse_moves("v<A<AA>>^AvA^<A>AAAvA^Av<A<A>>^AAA<Av>A^Av<A^>A<Av<A>>^AvA^Av<A<A>>^A<Av>A^A");
    //     debug_reverse_moves("v<A<AA>>^AvA^<A>AAvA^Av<A<A>>^A<Av>A^Av<<A>>^AAvA^Av<A^>Av<<A>>^AAA<Av>A^A");
    //     debug_reverse_moves("v<A<AA>>^AvA^<A>AAvA^Av<<A>>^AvA^Av<A^>Av<<A>>^A<Av>A^Av<A<A>>^AA<Av>A^A");
    //     debug_reverse_moves("v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AvA^Av<A<A>>^A<Av>A^Av<A^>AAv<<A>>^A<Av>A^A");
    //     debug_reverse_moves("v<<A>>^AvA^Av<A<AA>>^AAvAA^<A>Av<A^>AA<Av<A>>^AAvA^Av<A<A>>^AAA<Av>A^A");
    // }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(228800606998554, part2(test_input));
    }
}
