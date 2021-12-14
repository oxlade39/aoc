use std::{collections::{HashMap, HashSet}, slice::Chunks};


fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();
    let numbers: Vec<i32> = lines.next().unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let remaining = lines
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .collect::<Vec<_>>();
    let board_lines = remaining.chunks(5);

    // println!("part one winner: {:?}", part_one(numbers, parse_boards(board_lines)));
    println!("part two winner: {:?}", part_two(numbers, parse_boards(board_lines)));
}

fn parse_boards(boards: Chunks<&str>) -> Vec<Board> {
    let mut parsed_boards = vec![Board::default(); boards.len()];
    for (board_idx, board) in boards.enumerate() {
        for (row_num, row) in board.into_iter().enumerate() {
            for (col_num, col) in row.split(" ")
                .filter(|item| item.len() > 0)
                .enumerate() {

                let value = col.parse().unwrap();
                parsed_boards[board_idx].update(row_num, col_num, value);
            }
        }
    }
    parsed_boards
}

#[allow(dead_code)]
fn part_one(numbers: Vec<i32>, mut boards: Vec<Board>) -> Option<i32> {
    for n in numbers {
        for b in &mut boards {
            if let Some(winner) = b.remove(n) {
                return Some(winner * n);
            }
        }
    }
    None
}

fn part_two(numbers: Vec<i32>, mut boards: Vec<Board>) -> Option<i32> {
    let mut result = None;
    let mut count;
    let mut winning_boards = HashSet::with_capacity(boards.len());
    for n in numbers {
        count = 0;
        
        for b in &mut boards {
            if winning_boards.contains(&count) {
                count += 1;
                continue;
            }

            if let Some(winner) = b.remove(n) {
                result = Some(winner * n);
                winning_boards.insert(count);
                println!("board {} won with {:?}", count+1, result);
            }
            count += 1;
        }
    }
    result
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct BoardPosition {
    row: usize,
    col: usize
}

#[derive(Debug, Clone)]
struct Board {
    row_sums: [i32; 5],
    col_sums: [i32; 5],
    positions: HashMap<i32, BoardPosition>
}

impl Default for Board {
    fn default() -> Self {
        Self { row_sums: [0; 5], col_sums: [0; 5], positions: HashMap::with_capacity(5 * 5) }
    }
}

impl Board {
    fn update(&mut self, row: usize, col: usize, value: i32) {
        self.col_sums[col] += value;
        self.row_sums[row] += value;
        self.positions.insert(value, BoardPosition{col, row});
    }

    fn remove(&mut self, value: i32) -> Option<i32> {
        match self.positions.remove(&value) {
            Some(p) => {
                self.col_sums[p.col] -= value;
                self.row_sums[p.row] -= value;

                if self.col_sums[p.col] == 0 || self.row_sums[p.row] == 0 {
                    Some(self.positions.keys().sum())
                } else {
                    None
                }
            },
            None => None
        }
    }
}
