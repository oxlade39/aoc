use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let total: i32 = total_priorities(input);
    println!("total: {total}");

    let total_chunk: i32 = chunk_3(input);
    println!("part2: {total_chunk}")
}

fn char_priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as i32) - ('a' as i32) + 1
    } else {
        (c as i32) - ('A' as i32) + 27
    }
}

fn sum_intersection_priority(left: HashSet<char>, right: HashSet<char>) -> i32 {
    left
        .intersection(&right)
        .map(|c| char_priority(*c))
        .sum()
}

fn total_priorities(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            (
                left.chars().collect::<HashSet<char>>(),
                right.chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(left, right)| sum_intersection_priority(left, right))
        .sum()
}

fn chunk_3(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let items: Vec<_> = chunk.collect();
            let i: i32 = items[0]
                .intersection(&items[1])
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&items[2])
                .copied()
                .map(|c| char_priority(c))
                .sum();
            i
        })
        .sum()
}


#[test]
fn test_char_priority() {
    assert_eq!(1, char_priority('a'));
    assert_eq!(27, char_priority('A'));
    assert_eq!(52, char_priority('Z'));
}

#[test]
fn test_example_part1() {
    assert_eq!(157, total_priorities(include_str!("input.example.txt")))
}

#[test]
fn test_example_part2() {
    assert_eq!(70, chunk_3(include_str!("input.example.txt")))
}