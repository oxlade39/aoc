use core::str;
use std::{i64, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let mut locks: Vec<Schematic> = Vec::new();
    let mut keys: Vec<Schematic> = Vec::new();
    for block in input::empty_line_chunks(txt) {
        match block.parse::<Schematic>().unwrap() {
            l @ Schematic::Lock(_) => {
                locks.push(l);
            },
            k @ Schematic::Key(_) => {
                keys.push(k);
            },
        }
    }

    let mut counts = 0;
    for l in &locks {
        for k in &keys {
            if l.fits(k) {
                counts += 1;
            }
        }
    }
    counts
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Schematic {
    Lock([usize; 5]),
    Key([usize; 5])
}

impl Schematic {
    fn fits(&self, other: &Schematic) -> bool {
        match (self, other) {
            (Schematic::Lock(_), Schematic::Lock(_)) => return false,
            (Schematic::Key(_), Schematic::Key(_)) => return false,
            _ => {}
        }

        let my_heights = match self {
            Schematic::Lock(heights) => heights,
            Schematic::Key(heights) => heights,
        };
        let other_heights = match other {
            Schematic::Lock(heights) => heights,
            Schematic::Key(heights) => heights,
        };

        for i in 0..my_heights.len() {
            let total = my_heights[i] + other_heights[i];
            if total > 7 {
                return false;
            }
        }

        true
    }

    fn invert(&self) -> Schematic {
        match self {
            Schematic::Lock(heights) => {
                Schematic::Key([
                    7 - heights[0],
                    7 - heights[1],
                    7 - heights[2],
                    7 - heights[3],
                    7 - heights[4],
                ])
            },
            Schematic::Key(heights) => {
                Schematic::Lock([
                    7 - heights[0],
                    7 - heights[1],
                    7 - heights[2],
                    7 - heights[3],
                    7 - heights[4],
                ])
            },
        }
    }
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_lock = s.starts_with("#");
        let mut counts = [0_usize; 5];
        for line in s.lines() {
            let chars: Vec<_> = line.chars().collect();
            for i in 0..chars.len() {
                let is_hash = chars[i] == '#';
                if is_lock && is_hash {
                    counts[i] += 1;
                }
                if !is_lock && is_hash {
                    counts[i] += 1;
                }
            }            
        }

        if is_lock {
            Ok(Schematic::Lock(counts))
        } else {
            Ok(Schematic::Key(counts))
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::*;

    #[test]
    fn test_parse() {
        let test_input = include_str!("input.test.txt");
        let parsed: Vec<Schematic> = input::empty_line_chunks(test_input)
            .map(|block| block.parse().unwrap())
            .collect();
        assert_eq!(parsed, vec![
            Schematic::Lock([1, 6, 4, 5, 4]),
            Schematic::Lock([2, 3, 1, 6, 4]),
            Schematic::Key([6, 1, 3, 2, 4]),
            Schematic::Key([5, 4, 5, 1, 3]),
            Schematic::Key([4, 1, 3, 1, 2]),
        ])
    }

    #[test]
    fn test_invert() {
        let lock = Schematic::Lock([1, 6, 4, 5, 4]);
        let key = lock.invert();
        assert_eq!(key, Schematic::Key([
            6, 1, 3, 2, 3
        ]));
    }

    #[test]
    fn test_fits() {
        let lock = Schematic::Lock([1, 6, 4, 5, 4]);
        let key = Schematic::Key([6, 1, 3, 2, 4]);

        assert_eq!(false, lock.fits(&key));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(3, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(3021, part1(test_input));
    }
}
