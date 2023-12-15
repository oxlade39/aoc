use std::{collections::HashMap, time::Instant};

use aoclib::{input, number};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let parts: Vec<_> = input::empty_line_chunks(txt).collect();
    let directions: Vec<_> = parts[0].chars().collect();

    let map: HashMap<_, _> = parts[1]
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(" = ").collect();
            let key = parts[0];
            let replaced = parts[1].replace("(", "");
            let replaced = replaced.replace(")", "");
            let parts: Vec<_> = replaced.split(", ").collect();
            (key.to_owned(), (parts[0].to_owned(), parts[1].to_owned()))
        })
        .collect();

    let mut steps = 0;
    let mut current_location = &"AAA".to_owned();

    loop {
        let current_direction = directions[steps % directions.len()];
        let (left, right) = map.get(current_location).unwrap();
        current_location = match current_direction {
            'L' => left,
            'R' => right,
            _ => unreachable!("bad direction"),
        };
        steps += 1;

        if current_location == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(txt: &str) -> u64 {
    let parts: Vec<_> = input::empty_line_chunks(txt).collect();
    let directions: Vec<_> = parts[0].chars().collect();

    let mut map: HashMap<_, _> = HashMap::new();
    let mut start_nodes: Vec<_> = Vec::new();

    for l in parts[1].lines() {
        let parts: Vec<_> = l.split(" = ").collect();
        let key = parts[0];
        let replaced = parts[1].replace("(", "");
        let replaced = replaced.replace(")", "");
        let parts: Vec<_> = replaced.split(", ").collect();
        map.insert(key.to_owned(), (parts[0].to_owned(), parts[1].to_owned()));
        if key.chars().last().unwrap() == 'A' {
            start_nodes.push(key);
        }
    }

    let mut steps: Vec<u64> = Vec::with_capacity(start_nodes.len());

    for i in 0..start_nodes.len() {
        let mut my_steps: u64 = 0;
        let mut current_location = start_nodes[i];
        loop {
            let d: u64 = my_steps % directions.len() as u64;
            let current_direction = directions[d as usize];
            let (left, right) = map.get(current_location).unwrap();
            current_location = match current_direction {
                'L' => left,
                'R' => right,
                _ => unreachable!("bad direction"),
            };
            my_steps += 1;

            if current_location.chars().last().unwrap() == 'Z' {
                steps.push(my_steps);
                break;
            }
        }
    }
    println!("solved {:?}", steps);

    number::lcm(&steps[0..])
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(2, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p1_2() {
        assert_eq!(6, part1(include_str!("input.test2.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(6, part2(include_str!("input.test3.txt")));
    }
}
