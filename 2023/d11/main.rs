use std::{collections::HashSet, time::Instant, str::FromStr, i64};

use aoclib::{cartesian::Point, distance::{ManhattenDistance, Distance}};
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> i64 {
    calc(txt, 2)
}

fn part2(txt: &str) -> i64 {
    calc(txt, 1000000)
}

fn calc(txt: &str, expansion: i64) -> i64 {
    let map: Map = {
        let mut m: Map = txt.parse().unwrap();
        m.expand(expansion);
        m
    };
    map.pairs()
        .map(|pair| {
            ManhattenDistance::from_vector(pair.into())
        })
        .map(|d| d.0)
        .sum()
}

#[derive(Debug, Clone)]
struct Map {
    empty_x: HashSet<usize>,
    empty_y: HashSet<usize>,
    galaxies: Vec<Point>,
}

impl Map {
    fn pairs(self) -> impl Iterator<Item = (Point, Point)> {
        self.galaxies.into_iter()
            .combinations(2)
            .map(|mut comb| {
                (comb.pop().expect("left"), comb.pop().expect("right"))
            })
    }

    fn expand(&mut self, increment: i64) {
        let mut fixed_galaxies = Vec::with_capacity(self.galaxies.len());
        
        while let Some(g) = self.galaxies.pop() {

            let mut x_increment = 0;
            for x in 0..g.x {
                if self.empty_x.contains(&(x as usize)) {
                    x_increment += increment - 1;
                }
            }

            let mut y_increment = 0;
            for y in 0..g.y {
                if self.empty_y.contains(&(y as usize)) {
                    y_increment += increment - 1;
                }
            }

            fixed_galaxies.push(Point { 
                x: g.x + x_increment, 
                y: g.y + y_increment, 
            });
        }
        std::mem::swap(&mut self.galaxies, &mut fixed_galaxies);
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        let height = chars.len();
        let width = chars[0].len();

        let mut empty_x: HashSet<usize> = HashSet::from_iter(0..height);
        let mut empty_y: HashSet<usize> = HashSet::from_iter(0..width);
        let mut galaxies = Vec::new();

        for row in 0..height {
            let y = height - row - 1;
            for x in 0..width {
                let p: Point = (x as i64, y as i64).into();
                if chars[y][x] == '#' {
                    galaxies.push(p);
                    empty_y.remove(&y);
                    empty_x.remove(&x);
                }
            } 
        }
        Ok(Self { empty_x, empty_y, galaxies })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(374, part1(include_str!("input.test.txt")));
    }


    #[test]
    fn test_example_p2() {
        assert_eq!(8410, calc(include_str!("input.test.txt"), 100));
    }
}
