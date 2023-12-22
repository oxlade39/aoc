use std::collections::{HashMap, HashSet};

use aoclib::{
    cartesian::{Plane, Point},
    neighbour::Neighbours,
    neighbour::TouchingNeighbours,
};
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
        _ => true,
    }
}

fn part1(txt: &str) -> i32 {
    let grid = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();

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
                            counts = counts
                                || (row < (grid.len() - 1) && is_symbol(grid[row + 1][col - 1]));
                        }
                    }
                }
                other => {
                    if !num_chars.is_empty() {
                        // char itself could be a symbol
                        counts = counts || is_symbol(other);

                        // above
                        counts = counts || (row > 0 && is_symbol(grid[row - 1][col]));
                        // below
                        counts =
                            counts || (row < (grid.len() - 1) && is_symbol(grid[row + 1][col]));

                        if counts {
                            let num = num_chars.parse::<i32>().expect("numeric");
                            total += num;
                        }
                        num_chars.clear();
                        counts = false;
                    }
                }
            }
        }
        if counts && !num_chars.is_empty() {
            let num = num_chars.parse::<i32>().expect("numeric");
            total += num;
        }
        num_chars.clear();
    }
    total
}

fn part2(txt: &str) -> i32 {
    let grid = txt.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let plane: Plane = (grid.len() as i64, grid[0].len() as i64).into();

    let mut markers: HashMap<Point, HashSet<Number>> = HashMap::new();
    let mut numbers = Vec::new();

    for row in 0..grid.len() {
        let mut n = String::new();
        for col in 0..grid[0].len() {
            let c = grid[row][col];

            match c {
                other if other.is_numeric() => {
                    n.push(other);
                }
                other => {
                    if other == '*' {
                        let p = Point {
                            x: col as i64,
                            y: row as i64,
                        };
                        markers.insert(p, HashSet::new());
                    }

                    if !n.is_empty() {
                        let num = n.parse::<i32>().expect("number");
                        n.clear();
                        numbers.push(Number {
                            n: num,
                            right: Point {
                                x: (col - 1) as i64,
                                y: row as i64,
                            },
                        });
                    }
                }
            }
        }
        if !n.is_empty() {
            let num = n.parse::<i32>().expect("number");
            numbers.push(Number {
                n: num,
                right: Point {
                    x: (grid[0].len() - 1) as i64,
                    y: row as i64,
                },
            });
        }
    }

    for num in numbers {
        for n in num.neighbours(&plane) {
            if let Some(marker) = markers.get_mut(&n) {
                marker.insert(num.clone());
            }
        }
    }

    markers
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v.iter().map(|n| n.n).product::<i32>())
        .sum()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Number {
    n: i32,
    right: Point,
}

impl Number {
    fn left(&self) -> Point {
        let width = format!("{}", self.n).len();
        Point {
            x: self.right.x + 1 - (width as i64),
            y: self.right.y,
        }
    }

    fn neighbours(&self, plane: &Plane) -> HashSet<Point> {
        let mut result = HashSet::new();
        let n = TouchingNeighbours(plane);
        for x in self.left().x..=self.right.x {
            let current_point = Point { x, y: self.right.y };
            result.extend(n.neighbours(&current_point));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_left_and_right() {
        let n = Number {
            n: 114,
            right: Point { x: 7, y: 0 },
        };
        let left: Point = (5, 0).into();
        assert_eq!(left, n.left());
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(4361, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(467835, part2(include_str!("input.test.txt")));
    }
}
