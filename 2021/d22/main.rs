use std::time::Instant;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {}ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");

    let mut grid = vec![vec![vec![false; 101]; 101]; 101];
    let mut count: i64 = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(" ").collect();
        let on_off = parts[0];
        let mut axis: Vec<_> = parts[1].split(",").collect();
        let (start_z, end_z) = parse_parts(axis.pop().unwrap());
        let (start_y, end_y) = parse_parts(axis.pop().unwrap());
        let (start_x, end_x) = parse_parts(axis.pop().unwrap());

        if start_z < 0 || end_z > 100 || start_y < 0 || end_y > 100 || start_x < 0 || start_x > 100
        {
            println!("bad line: {}", line);
            continue;
        }

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                for z in start_z..=end_z {
                    let new_value = on_off == "on";
                    let existing_value = grid[x as usize][y as usize][z as usize];

                    if new_value != existing_value {
                        grid[x as usize][y as usize][z as usize] = new_value;

                        if new_value {
                            count += 1;
                        } else {
                            count -= 1;
                        }
                    }
                }
            }
        }
    }
    println!("pt1 result: {}", count);
}

fn parse_parts(axis_str: &str) -> (i64, i64) {
    let axis_and_range: Vec<_> = axis_str.split("=").collect();
    let split: Vec<_> = axis_and_range[1].split("..").collect();
    let start: i64 = split[0].parse().unwrap();
    let end: i64 = split[1].parse().unwrap();
    (start + 50, end + 50)
}

fn part2() {
    let input = include_str!("input.txt");
    println!("result: {}", 1);
}
