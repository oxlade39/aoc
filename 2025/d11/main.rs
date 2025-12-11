use core::str;
use std::{i64, time::Instant};

use aoclib::timing;
use hashbrown::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let connections: HashMap<String, Vec<String>> = txt
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let key = parts.next().expect("key");
            let key = &key[0..key.len() - 1];
            let values: Vec<_> = parts.map(|s| s.to_owned()).collect();
            (key.to_owned(), values)
        })
        .collect();

    // println!("connections: {:?}", connections);

    let mut memo = HashMap::new();
    memo.insert("out".to_owned(), 1);
    count("you".to_owned(), &connections, &mut memo)
}

fn part2(txt: &str) -> i64 {
    let connections: HashMap<String, Vec<String>> = txt
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let key = parts.next().expect("key");
            let key = &key[0..key.len() - 1];
            let values: Vec<_> = parts.map(|s| s.to_owned()).collect();
            (key.to_owned(), values)
        })
        .collect();

    count_between("svr".to_owned(), "fft".to_owned(), &connections)
        * count_between("fft".to_owned(), "dac".to_owned(), &connections)
        * count_between("dac".to_owned(), "out".to_owned(), &connections)
}

fn count(
    current: String,
    connections: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if let Some(existing) = memo.get(&current) {
        return *existing;
    }

    if let Some(next) = connections.get(&current) {
        let mut sum = 0;
        for n in next {
            sum += count(n.clone(), connections, memo);
        }
        memo.insert(current, sum);
        sum
    } else {
        memo.insert(current, 0);
        0
    }
}

fn count_between(left: String, right: String, connections: &HashMap<String, Vec<String>>) -> i64 {
    let mut fft_to_dac = HashMap::new();
    fft_to_dac.insert(right, 1);
    count(left, &connections, &mut fft_to_dac)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(5, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(413, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input2.test.txt");
        assert_eq!(2, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(525518050323600, part2(test_input));
    }
}
