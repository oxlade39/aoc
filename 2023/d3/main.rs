use std::collections::{HashSet, HashMap};

use aoclib::{cartesian::{Point, Plane}, astar::TouchingNeighbours, astar::Neighbours};
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn is_symbol(c: char) -> bool {
    match c {
        '.' => false,
        other if other.is_numeric() => false,
        _ => true
    }
}

fn part1(txt: &str) -> i32 {

    let grid = txt.lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let mut total = 0;
    for row in 0..grid.len() {
        let mut num_chars = String::new();
        let mut counts = false;
        for col in 0..grid[0].len() {
            let c = grid[row][col];
            match c {
                num if num.is_numeric() => {
                    // add the char
                    num_chars.push(num);

                    // above
                    counts = counts || (row > 0 && is_symbol(grid[row - 1][col]));
                    // below
                    counts = counts || (row < (grid.len() - 1) && is_symbol(grid[row + 1][col]));


                    if num_chars.len() == 1 {
                        // check points behind
                        if col > 0 {
                            counts = counts || (row > 0 && is_symbol(grid[row - 1][col - 1]));
                            counts = counts || is_symbol(grid[row][col - 1]);
                            counts = counts || (row < (grid.len() - 1) && is_symbol(grid[row + 1][col - 1]));
                        }
                    }


                },
                other => {
                    if !num_chars.is_empty() {

                        // char itself could be a symbol
                        counts = counts || is_symbol(other);

                        // above
                        counts = counts || (row > 0 && is_symbol(grid[row - 1][col]));
                        // below
                        counts = counts || (row < (grid.len() - 1) && is_symbol(grid[row + 1][col]));

                        if counts {
                            let num = num_chars.parse::<i32>().expect("numeric");
                            println!("adding: {}", num);
                            total += num;
                        }
                        num_chars.clear();
                        counts = false;
                    }
                },
            }
        }
        if counts && !num_chars.is_empty() {
            let num = num_chars.parse::<i32>().expect("numeric");
            println!("adding: {}", num);
            total += num;
        }
        num_chars.clear();
    }
    total
}

fn part2(txt: &str) -> i32 {
    -1
}


mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(4361, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(-1, part2(include_str!("input.test.txt")));
    }
}
