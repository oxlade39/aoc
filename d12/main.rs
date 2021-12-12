use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    str::FromStr,
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.test.txt");

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split("-").collect();
        let left = parts[0].to_string();
        let right = parts[1].to_string();

        if let Some(it) = connections.get_mut(&left) {
            it.push(right.clone());
        } else {
            let mut v = Vec::new();
            v.push(right.clone());
            connections.insert(left.clone(), v);
        }

        if left != "start" {
            if let Some(it) = connections.get_mut(&right) {
                it.push(left.clone());
            } else {
                let mut v = Vec::new();
                v.push(left.clone());
                connections.insert(right, v);
            }
        }
    }

    println!("mappings: {:?}", connections);

    let result = count(
        "start".to_string(),
        &connections,
        &mut Vec::new(),
        &mut HashMap::new(),
    );
    println!("part1: {:?}", result);
}

fn count(
    curr: String,
    mappings: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    let is_lower = curr.to_lowercase() == curr;

    if is_lower && path.contains(&curr) {
        println!("seen lower {:?},{}", path, curr);
        0
    } else {
        if curr == "end" {
            println!("[{:?}] -> end", path);
            1
        } else {
            path.push(curr.clone());

            if let Some(seen) = memo.get(&curr) {
                println!("{:?} memo => {}", path, seen);
                *seen
            } else {

                let children = mappings.get(&curr).unwrap();
                let mut child_count = 0;
                for child in children {
                    let mut path_cp = path.clone();
                    let c = count(child.clone(), mappings, &mut path_cp, memo);
                    child_count += c;
                    println!("child {:?},{} => {}", path_cp, child, c);
                }
                // memo.insert(curr.clone(), child_count);
                println!("parent {:?} => {}", path, child_count);
                child_count
            }
        }
    }
}

fn part2() {
    let input = include_str!("input.txt");
}
