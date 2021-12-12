use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let connections = parse(input);
    
    let result = count(
        "start".to_string(),
        &connections,
        &mut Vec::new(),
    );
    println!("part1: {:?}", result);
}

fn part2() {
    pt2(include_str!("simple.test.txt"));
    pt2(include_str!("slightly_larger.test.txt"));
    pt2(include_str!("input.test.txt"));
    pt2(include_str!("input.txt"));
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split("-").collect();
        let left = parts[0].to_string();
        let right = parts[1].to_string();

        if right != "start" {
            if let Some(it) = connections.get_mut(&left) {
                it.push(right.clone());
            } else {
                let mut v = Vec::new();
                v.push(right.clone());
                connections.insert(left.clone(), v);
            }
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

    connections
}

fn count(
    curr: String,
    mappings: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
) -> i32 {
    let is_lower = curr.to_lowercase() == curr;

    if is_lower && path.contains(&curr) {
        0
    } else {
        if curr == "end" {
            1
        } else {
            path.push(curr.clone());
            let children = mappings.get(&curr).unwrap();
            let mut child_count = 0;
            for child in children {
                let mut path_cp = path.clone();
                let c = count(child.clone(), mappings, &mut path_cp);
                child_count += c;
            }
            child_count
        }
    }
}

fn pt2(input: &str) {
    let connections = parse(input);
    
    let result = count2(
        "start".to_string(),
        &connections,
        &mut Vec::new(),
    );
    println!("part2: {:?}", result);
}

fn count2(
    curr: String,
    mappings: &HashMap<String, Vec<String>>,
    smalls: &mut Vec<String>,
) -> i32 {
    let is_lower = curr.to_lowercase() == curr;

    let unique_small: HashSet<String> = HashSet::from_iter(smalls.clone());
    let already_two = unique_small.len() < smalls.len();

    if is_lower && already_two && unique_small.contains(&curr) { 
        0
    } else {
        if curr == "end" {
            1
        } else {
            if is_lower {
                smalls.push(curr.clone());
            }            
            let children = mappings.get(&curr).unwrap();
            let mut child_count = 0;
            for child in children {
                let mut path_cp = smalls.clone();
                let c = count2(child.clone(), mappings, &mut path_cp);
                child_count += c;
            }
            child_count
        }
    }
}