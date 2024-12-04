use core::str;
use std::time::Instant;

use aoclib::{grid::{Grid, GridPosition}, input};

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
    let forward = "XMAS";
    let backwards = "SAMX";

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
                let string = String::from_iter(down);
                let s = string.as_str();
                if s == forward || s == backwards {
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

                let string = String::from_iter(right);
                let s = string.as_str();
                if s == forward || s == backwards {
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

                let string = String::from_iter(down_right);
                let s = string.as_str();
                if s == forward || s == backwards {
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

                let string = String::from_iter(down_left);
                let s = string.as_str();
                if s == forward || s == backwards {
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

    println!("grid:\n{}\n", g);


    let possible_1 = [
        'M', '.', 'S',
        '.', 'A', '.',
        'M', '.', 'S',
    ];
    let possible_2 = [
        'S', '.', 'S',
        '.', 'A', '.',
        'M', '.', 'M',
    ];
    let possible_3 = [
        'S', '.', 'M',
        '.', 'A', '.',
        'S', '.', 'M',
    ];
    let possible_4 = [
        'M', '.', 'M',
        '.', 'A', '.',
        'S', '.', 'S',
    ];

    let mut count = 0;

    for row in 0..(g.height() - 2) {
        for col in 0..(g.width() - 2) {
            let p = GridPosition::new(col, row);

            let inner = [
                *g.at(&p), '.', *g.at(&p.right().right()),
                '.', *g.at(&p.down().right()), '.',
                *g.at(&p.down().down()), '.', *g.at(&p.down().down().right().right()),
            ];

            println!("@ {:?}", p);
            println!("{:?}{:?}{:?}", inner[0], inner[1], inner[2]);
            println!("{:?}{:?}{:?}", inner[3], inner[4], inner[5]);
            println!("{:?}{:?}{:?}", inner[6], inner[7], inner[8]);
            println!("");

            if inner == possible_1 {
                println!("possible 1");
                println!("{:?}{:?}{:?}", possible_1[0], possible_1[1], possible_1[2]);
                println!("{:?}{:?}{:?}", possible_1[3], possible_1[4], possible_1[5]);
                println!("{:?}{:?}{:?}", possible_1[6], possible_1[7], possible_1[8]);

                println!("");
                count += 1;
            }

            if inner == possible_2 {
                println!("possible 2");
                println!("{:?}{:?}{:?}", possible_2[0], possible_2[1], possible_2[2]);
                println!("{:?}{:?}{:?}", possible_2[3], possible_2[4], possible_2[5]);
                println!("{:?}{:?}{:?}", possible_2[6], possible_2[7], possible_2[8]);
                
                println!("");
                count += 1;
            }

            if inner == possible_3 {
                println!("possible 3");
                println!("{:?}{:?}{:?}", possible_3[0], possible_3[1], possible_3[2]);
                println!("{:?}{:?}{:?}", possible_3[3], possible_3[4], possible_3[5]);
                println!("{:?}{:?}{:?}", possible_3[6], possible_3[7], possible_3[8]);

                println!("");
                count += 1;
            }

            if inner == possible_4 {
                println!("possible 4");
                println!("{:?}{:?}{:?}", possible_4[0], possible_4[1], possible_4[2]);
                println!("{:?}{:?}{:?}", possible_4[3], possible_4[4], possible_4[5]);
                println!("{:?}{:?}{:?}", possible_4[6], possible_4[7], possible_4[8]);

                println!("");
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
        assert_eq!(0, part2(test_input));
    }
}
