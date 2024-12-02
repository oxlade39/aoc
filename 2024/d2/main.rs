use std::{iter::zip, time::Instant};

use aoclib::grid::{Grid, GridPosition};
use itertools::Itertools;

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
    let valid = txt.lines().filter_map(|l| {
        l.split_whitespace().fold(Some(RowState::new()), |accum, rs| {
            let n: i32 = rs.parse().unwrap();
            match accum {
                Some(RowState { prev, prev_delta, }) => {
                    match (prev, prev_delta) {
                        (Some(p), Some(pd)) => {
                            let diff = n - p;

                            if diff.signum() != pd.signum() || diff.abs() < 1 || diff.abs()> 3 {
                                return None;
                            }
        
                            Some(RowState { prev: Some(n), prev_delta: Some(diff) })
                        },
                        (Some(p), None) => {
                            let diff = n - p;
                            if diff.abs() < 1 || diff.abs() > 3 {
                                return None;
                            }
        
                            Some(RowState { prev: Some(n), prev_delta: Some(diff) })
                        }
                        (None, None) => Some(RowState { prev: Some(n), prev_delta: None }),
                        (None, Some(_)) => panic!("impossible"),                        
                    }                    
                },
                None => None,
            }
        })
    });

    valid.count()
}

fn part2(txt: &str) -> usize {
    0
}

#[derive(Debug)]
struct RowState {
    prev: Option<i32>,
    prev_delta: Option<i32>,
}

impl RowState {
    fn new() -> Self {
        RowState { prev: None, prev_delta: None }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sample_input_pt1_lines() {
        assert_eq!(1, part1("7 6 4 2 1"));
    }
    
    #[test]
    fn sample_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2, part1(test_input));
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
