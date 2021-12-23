use core::panic;
use std::{time::Instant, collections::HashSet, hash::Hash, fmt::Debug, iter::repeat};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");

    let player_one = input.lines().nth(0).unwrap().chars().last().unwrap().to_digit(10).unwrap() as i64;
    let player_two = input.lines().nth(1).unwrap().chars().last().unwrap().to_digit(10).unwrap() as i64;

    let mut rolls = 0;
    let mut scores: [i64; 2] = [0, 0];
    let mut positions: [i64; 2] = [player_one, player_two];
    println!("starting on {:?}", positions);
    loop {

        rolls += 3;
        let max_die = ((rolls - 1) % 100) + 1;
        let roll_one_score = max_die + (max_die - 1) + (max_die - 2);
        positions[0] = (((positions[0] - 1) + roll_one_score) % 10) + 1;
        scores[0] += positions[0];
        println!("1 to pos. {} with score: {}", positions[0], roll_one_score);

        if scores[0] >= 1000 {
            break;
        }
        
        rolls += 3;
        let max_die = ((rolls - 1) % 100) + 1;
        let roll_two_score = max_die + (max_die - 1) + (max_die - 2);
        positions[1] = (((positions[1] - 1) + roll_two_score) % 10) + 1;
        scores[1] += positions[1];
        println!("2 to pos. {} with score: {}", positions[1], roll_two_score);

        if scores[1] >= 1000 {
            break;
        }

        println!("2 to {}", positions[1]);
        println!("rolls: {}: {:?}", rolls, scores);
    }

    println!("rolls: {}", rolls);
    let min_score = i64::min(scores[0], scores[1]);
    println!("min score: {}", min_score);
    println!("pt1: {}", min_score * rolls);
}

fn part2() {
    let input = include_str!("input.txt");
}

fn triangle_number(n: i64) -> i64 {
    ((n + 1) * n) / 2
}
