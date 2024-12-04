use core::str;
use std::time::Instant;

use aoclib::grid::{Grid, GridPosition};

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

fn part1(txt: &str) -> i64 {
    let forward = ['X', 'M', 'A', 'S'];
    let backwards = ['S', 'A', 'M', 'X'];

    let g: Grid<char> = txt.parse().unwrap();
    let mut count = 0;

    for row in 0..g.height() {
        for col in 0..g.width() {
            let p = GridPosition::new(col, row);

            let room_down = row + 3 < g.height();
            let room_right = col + 3 < g.width();
            let room_left = col >= 3;

            if room_down {
                // down
                let down = [
                    *g.at(&p),
                    *g.at(&p.down()),
                    *g.at(&p.down().down()),
                    *g.at(&p.down().down().down()),
                ];
                if (down == forward) || (down == backwards) {
                    count += 1;
                }
            }

            if room_right {
                // right
                let right = [
                    *g.at(&p),
                    *g.at(&p.right()),
                    *g.at(&p.right().right()),
                    *g.at(&p.right().right().right()),
                ];

                if (right == forward) || (right == backwards) {
                    count += 1;
                }
            }

            if room_down && room_right {
                // down right
                let down_right = [
                    *g.at(&p),
                    *g.at(&p.down().right()),
                    *g.at(&p.down().right().down().right()),
                    *g.at(&p.down().right().down().right().down().right()),
                ];

                if (down_right == forward) || (down_right == backwards) {
                    count += 1;
                }
            }

            if room_down && room_left {
                // down left
                let down_left = [
                    *g.at(&p),
                    *g.at(&p.down().left()),
                    *g.at(&p.down().left().down().left()),
                    *g.at(&p.down().left().down().left().down().left()),
                ];

                if (down_left == forward) || (down_left == backwards) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(txt: &str) -> i64 {
    fn not_mas(c: char) -> bool {
        if !c.is_alphabetic() {
            return false;
        }
        match c {
            'M' | 'A' | 'S' => false,
            _ => true,
        }
    }

    let interesting = txt.replace(not_mas, ".");
    let g: Grid<char> = interesting.parse().unwrap();

    let possible_1 = ['M', '.', 'S', '.', 'A', '.', 'M', '.', 'S'];
    let possible_2 = ['S', '.', 'S', '.', 'A', '.', 'M', '.', 'M'];
    let possible_3 = ['S', '.', 'M', '.', 'A', '.', 'S', '.', 'M'];
    let possible_4 = ['M', '.', 'M', '.', 'A', '.', 'S', '.', 'S'];

    let mut count = 0;

    for row in 0..(g.height() - 2) {
        for col in 0..(g.width() - 2) {
            let p = GridPosition::new(col, row);

            let inner = [
                *g.at(&p),
                '.',
                *g.at(&p.right().right()),
                '.',
                *g.at(&p.down().right()),
                '.',
                *g.at(&p.down().down()),
                '.',
                *g.at(&p.down().down().right().right()),
            ];

            if inner == possible_1 {
                count += 1;
            }

            if inner == possible_2 {
                count += 1;
            }

            if inner == possible_3 {
                count += 1;
            }

            if inner == possible_4 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(18, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(2496, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(9, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1967, part2(test_input));
    }
}
