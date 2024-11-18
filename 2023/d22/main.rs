use std::{cmp::Ordering, str::FromStr, time::Instant};

use aoclib::range::Range;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let mut bricks: Vec<Brick> = txt.lines().map(|l| l.parse().unwrap()).sorted().collect();
    let collisions = drop_down(&mut bricks);

    let mut count = 0;
    for i in 0..bricks.len() {
        let supports = &collisions.supports[i];

        if supports.len() == 0 {
            count += 1;
        } else {
            let can_remove = supports
                .iter()
                .all(|i_support| collisions.supports_me[*i_support].len() > 1);
            if can_remove {
                count += 1;
            }
        }
    }

    count
}

fn part2(_txt: &str) -> usize {
    0
}

fn drop_down(bricks: &mut Vec<Brick>) -> Collisions {
    let mut collisions = Collisions::new(bricks.len());
    for i in 0..bricks.len() {
        loop {
            if bricks[i].z().from() == 1 {
                break;
            }

            let moved_down = bricks[i].down();
            let mut collided = false;
            for (j, _) in bricks[0..i]
                .iter()
                .enumerate()
                .filter(|(_, below)| moved_down.overlaps(below))
            {
                collided = true;
                collisions.add_collision(i, j);
            }

            if collided {
                break;
            }
            *&mut bricks[i] = moved_down;
        }
    }
    // not sure if needed. I think bricks might be able to change order
    // by sliding down the side without coliding
    // bricks.sort();
    collisions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick([usize; 6]);

impl Brick {
    fn x(&self) -> Range {
        return Range::new(self.0[0].min(self.0[3]), self.0[0].max(self.0[3]) + 1);
    }

    fn y(&self) -> Range {
        return Range::new(self.0[1].min(self.0[4]), self.0[1].max(self.0[4]) + 1);
    }

    fn z(&self) -> Range {
        return Range::new(self.0[2].min(self.0[5]), self.0[2].max(self.0[5]) + 1);
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.x().overlaps(other.x()) && self.y().overlaps(other.y()) && self.z().overlaps(other.z())
    }

    fn down(&self) -> Self {
        Self([
            self.0[0],
            self.0[1],
            self.0[2] - 1,
            self.0[3],
            self.0[4],
            self.0[5] - 1,
        ])
    }
}

#[derive(Debug)]
struct Collisions {
    supports_me: Vec<Vec<usize>>,
    supports: Vec<Vec<usize>>,
}

impl Collisions {
    fn new(size: usize) -> Self {
        Self {
            supports_me: vec![vec![]; size],
            supports: vec![vec![]; size],
        }
    }

    fn add_collision(&mut self, brick: usize, onto: usize) {
        self.supports_me[brick].push(onto);
        self.supports[onto].push(brick);
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z().from().cmp(&other.z().from())
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<_> = s
            .replace("~", ",")
            .split(",")
            .map(|i| i.parse::<usize>().unwrap())
            .collect();
        Ok(Self([
            items[0], items[1], items[2], items[3], items[4], items[5],
        ]))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(5, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse_text_input() {
        let txt = include_str!("input.test.txt");
        let parsed: Vec<Brick> = txt
            .lines()
            .map(|l| l.parse().unwrap())
            .sorted_unstable()
            .collect();

        assert_eq!(Brick([1, 0, 1, 1, 2, 1]), parsed[0]);
        assert_eq!(Brick([1, 1, 8, 1, 1, 9]), parsed[parsed.len() - 1]);
    }

    #[test]
    fn test_overlaps() {
        let a = Brick([1, 0, 1, 1, 2, 1]);
        let b = Brick([1, 1, 8, 1, 1, 9]);

        assert_eq!(true, a.overlaps(&a));
        assert_eq!(false, a.overlaps(&b));

        assert_eq!(false, a.down().overlaps(&a));
        assert_eq!(true, a.down().overlaps(&a.down()));
    }

    #[test]
    fn test_drop_test_input() {
        let txt = include_str!("input.test.txt");
        let mut parsed: Vec<Brick> = txt
            .lines()
            .map(|l| l.parse().unwrap())
            .sorted_unstable()
            .collect();
        let col = drop_down(&mut parsed);

        assert_eq!(1, parsed[0].z().from());
        assert_eq!(2, parsed[1].z().from());
        assert_eq!(2, parsed[2].z().from());
        assert_eq!(3, parsed[3].z().from());
        assert_eq!(3, parsed[4].z().from());
        assert_eq!(4, parsed[5].z().from());
        assert_eq!(5, parsed[6].z().from());
        assert_eq!(7, parsed[6].z().to()); // + 1 of actual because of range bounds

        println!("{:?}", col);
    }
}
