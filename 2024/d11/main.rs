use core::str;
use std::{time::Instant, usize};

use aoclib::timing;
use hashbrown::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let stones = txt.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .map(Stone)
        .collect_vec();
    count_all(25, stones)
}

fn part2(txt: &str) -> usize {
    let stones = txt.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .map(Stone)
        .collect_vec();
    count_all(75, stones)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Stone(usize);

impl Stone {
    fn n_digits(&self) -> usize {
        let f = (self.0 as f64).log10().floor();
        f as usize + 1
    }

    fn split(&self) -> (Stone, Stone) {        
        let digits = self.n_digits();

        // Compute the divisor for splitting
        let half_digits = digits / 2;
        let divisor = 10_u64.pow(half_digits as u32);
    
        // Split the number
        let first_half = self.0 as u64 / divisor;
        let second_half = self.0 as u64 % divisor;
    
        (Self(first_half as usize), Self(second_half as usize))
    }

    fn next(&self) -> Split {
        if self.0 == 0 {
            return Split::Stone(Stone(1));
        }
        if self.n_digits() % 2 == 0 {
            let (left, right) = self.split();
            return Split::Pair(left, right);
        }
        Split::Stone(Stone(self.0 * 2024))
    }
}

enum Split {
    Stone(Stone),
    Pair(Stone, Stone),
}

fn count_all(n: usize, stones: Vec<Stone>) -> usize {
    let mut memo = HashMap::new();
    stones.iter().map(|&s| count_n(s, 0, n, &mut memo)).sum()
}

fn count_n(
    s: Stone, 
    count: usize,
    depth: usize,
    memo: &mut HashMap<(Stone, usize), usize>
) -> usize {
    if let Some(&mem) = memo.get(&(s, depth)) {
        return mem;
    }

    if depth == 0 {
        return count + 1;
    }

    let sum = match s.next() {
        Split::Stone(stone) => count_n(stone, count, depth - 1, memo),
        Split::Pair(left, right) => count_n(left, count, depth - 1, memo) + count_n(right, count, depth - 1, memo),
    };
    memo.insert((s, depth), sum);
    sum
}

#[cfg(test)]
mod tests {    
    use itertools::Itertools;

    use crate::*;

    #[test]
    fn test_digits() {
        let s = Stone(123);
        assert_eq!(3, s.n_digits());
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(55312, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        let stones = test_input.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .map(Stone)
            .collect_vec();

        assert_eq!(233875, part1(test_input));
        assert_eq!(233875, count_all(25, stones));
    }

    #[test]
    fn input_test_pt2() {
        let test_input = include_str!("input.test.txt");
        let stones = test_input.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .map(Stone)
            .collect_vec();

        assert_eq!(22, count_all(6, stones.clone()));
        assert_eq!(55312, count_all(25, stones.clone()));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        let stones = test_input.split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .map(Stone)
            .collect_vec();
        assert_eq!(277444936413293, count_all(75, stones));
    }
}
