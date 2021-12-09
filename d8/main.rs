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
    let mut numbers: Vec<HashSet<&str>> = Vec::with_capacity(10);
    numbers[0] = HashSet::from_iter(["a", "b", "c", "e", "f", "g"]);
    numbers[1] = HashSet::from_iter(["c", "f"]);
    numbers[2] = HashSet::from_iter(["a", "c", "d", "e", "g"]);
    numbers[3] = HashSet::from_iter(["a", "c", "d", "f", "g"]);
    numbers[4] = HashSet::from_iter(["b", "d", "c", "f"]);
    numbers[5] = HashSet::from_iter(["a", "b", "d", "f", "g"]);
    numbers[6] = HashSet::from_iter(["a", "b", "d", "e", "f", "g"]);
    numbers[7] = HashSet::from_iter(["a", "c", "f"]);
    numbers[8] = HashSet::from_iter(["a", "b", "c", "d", "e", "f", "g"]);
}
