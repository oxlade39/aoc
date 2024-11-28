use std::{collections::HashSet, str::FromStr, time::Instant};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!(
        "{:.2}ms",
        (now.elapsed().subsec_nanos() as f32) / 1_000_000 as f32
    );
}

fn part1(txt: &str) -> usize {
    txt.lines()
        .map(|l| l.parse::<BoardingPass>().unwrap())
        .map(|pass| pass.seat(128, 8))
        .map(|seat_position| seat_position.seat_id())
        .max()
        .unwrap()
}

fn part2(txt: &str) -> usize {
    let all_seat_ids: HashSet<usize> = (0..843).collect();

    let seats_with_tickets: HashSet<usize> = txt
        .lines()
        .map(|l| l.parse::<BoardingPass>().unwrap())
        .map(|pass| pass.seat(128, 8))
        .map(|seat| seat.seat_id())
        .collect();

    all_seat_ids
        .difference(&seats_with_tickets)
        .copied()
        .filter(|&candidate| candidate > 0)
        .filter(|&candidate| seats_with_tickets.contains(&(candidate - 1)))
        .filter(|&candidate| seats_with_tickets.contains(&(candidate + 1)))
        .next()
        .unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoardingPass {
    row_partitions: [RowPartition; 7],
    col_partitions: [ColPartition; 3],
}

impl BoardingPass {
    fn seat(&self, rows: usize, cols: usize) -> SeatPosition {
        let mut row_range = 0..rows;
        let mut col_range = 0..cols;

        for row in &self.row_partitions {
            match row {
                RowPartition::Front => {
                    row_range = row_range.start..(row_range.end - (row_range.len() / 2))
                }
                RowPartition::Back => {
                    row_range = (row_range.start + (row_range.len() / 2))..row_range.end
                }
            }
            // dbg!(&row, &row_range);
        }
        for col in &self.col_partitions {
            match col {
                ColPartition::Left => {
                    col_range = col_range.start..(col_range.end - (col_range.len() / 2))
                }
                ColPartition::Right => {
                    col_range = (col_range.start + (col_range.len() / 2))..col_range.end
                }
            }
            // dbg!(&col, &col_range);
        }
        SeatPosition {
            row: row_range.start,
            col: col_range.start,
        }
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut row_itr = l[0..7].chars().map(|c| match c {
            'F' => RowPartition::Front,
            'B' => RowPartition::Back,
            bad => panic!("bad row partion '{}'", bad),
        });
        let mut col_itr = l[7..].chars().map(|c| match c {
            'L' => ColPartition::Left,
            'R' => ColPartition::Right,
            bad => panic!("bad col partion '{}'", bad),
        });
        let row_partitions = [
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
            row_itr.next().unwrap(),
        ];
        let col_partitions = [
            col_itr.next().unwrap(),
            col_itr.next().unwrap(),
            col_itr.next().unwrap(),
        ];
        Ok(BoardingPass {
            row_partitions,
            col_partitions,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SeatPosition {
    row: usize,
    col: usize,
}

impl SeatPosition {
    fn seat_id(&self) -> usize {
        // Every seat also has a unique seat ID:
        // multiply the row by 8, then add the column.
        self.row * 8 + self.col
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum RowPartition {
    Front,
    Back,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ColPartition {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn specific_example() {
        let txt = "FBFBBFFRLR";
        let bp: BoardingPass = txt.parse().unwrap();
        let s = bp.seat(128, 8);
        assert_eq!(SeatPosition { row: 44, col: 5 }, s);
        assert_eq!(357, s.seat_id());
    }

    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(820, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(842, part1(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(842, part1(test_input));
    }
}
