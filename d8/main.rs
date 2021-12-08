use std::{collections::HashSet, slice::Split};


fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let uniq_vec = vec![2, 4, 3, 7];
    let unique: HashSet<_> = HashSet::from_iter(uniq_vec.iter());
    let mut count = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(" | ").collect();
        println!("parts: {:?}", parts);
        let _signal = parts[0];
        let output = parts[1];
        println!("output: {:?}", output);
        let items = output.split(" ")
            .map(|item| item.len() as i32)
            .filter(|count| unique.contains(count))
            .count();
        count += items as i32;        
    }
    
    println!("count: {}", count);    
}

fn part2() {
}
