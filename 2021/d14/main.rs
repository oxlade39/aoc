use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");

    let chain = input.lines().nth(0).unwrap();
    let mut insertions: HashMap<[char; 2], Insertion> = HashMap::new();

    for line in input.lines().skip(2) {
        let i: Insertion = line.parse().unwrap();
        insertions.insert(i.chain, i);
    }

    let mut input: Vec<_> = chain.chars().collect();
    let n = 10;
    for _ in 0..n {
        input = step(&input, &insertions);
    }

    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in &input {
        if let Some(existing) = counts.insert(c.clone(), 1) {
            counts.insert(c.clone(), existing + 1);
        }
    }

    let (_, max_count) = counts
        .iter()
        .max_by(|left, right| left.1.cmp(right.1))
        .unwrap();

    let (_, min_count) = counts
        .iter()
        .min_by(|left, right| left.1.cmp(right.1))
        .unwrap();

    println!("after n {}: len: {:?}", n, input.len());
    println!("pt1: {}", max_count - min_count);
}

fn step(input: &Vec<char>, insertions: &HashMap<[char; 2], Insertion>) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    result.push(input[0]);
    for chunk in input.windows(2) {
        if let Some(insertion) = insertions.get(chunk) {
            let next = insertion.insert();
            result.push(next[1]);
            result.push(next[2]);
        }
    }
    result
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Insertion {
    chain: [char; 2],
    element: char,
}

impl Insertion {
    fn insert(&self) -> [char; 3] {
        [self.chain[0], self.element, self.chain[1]]
    }
}

#[derive(Debug)]
struct ParseErr;

impl FromStr for Insertion {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" -> ").collect();
        let c: Vec<_> = parts[0].chars().collect();
        let element = parts[1].chars().nth(0).unwrap();
        Ok(Insertion {
            chain: [c[0], c[1]],
            element,
        })
    }
}

fn part2() {
    let input = include_str!("input.txt");

    let chain = input.lines().nth(0).unwrap();
    let mut insertions: HashMap<[char; 2], Insertion> = HashMap::new();

    for line in input.lines().skip(2) {
        let i: Insertion = line.parse().unwrap();
        insertions.insert(i.chain, i);
    }

    let mut pairs: HashMap<[char; 2], i64> = HashMap::new();
    let input: Vec<_> = chain.chars().collect();
    for pair in input.windows(2) {
        let left = pair[0];
        let right = pair[1];
        let item = [left, right];
        if let Some(existing) = pairs.insert(item, 1) {
            pairs.insert(item, existing + 1);
        } else {
            pairs.insert(item, 1);
        }
    }

    let n = 40;

    for _ in 0..n {
        pairs = step2(&pairs, &insertions);
    }

    let mut counts: HashMap<char, i64> = HashMap::new();
    let mut all_chars: HashSet<char> = HashSet::new();

    for item in pairs.keys() {
        all_chars.insert(item[0]);
        all_chars.insert(item[1]);
    }

    for c in all_chars {
        let mut sum = if chain.ends_with(c) { 1 } else { 0 };
        for k in pairs.keys() {
            if k[0] == c {
                sum += pairs.get(k).unwrap();
            }
        }
        counts.insert(c, sum);
    }

    let (max, max_count) = counts
        .iter()
        .max_by(|left, right| left.1.cmp(right.1))
        .unwrap();

    let (min, min_count) = counts
        .iter()
        .min_by(|left, right| left.1.cmp(right.1))
        .unwrap();

    println!(
        "pt2: {} ({} -> {}, {} -> {})",
        max_count - min_count,
        min,
        min_count,
        max,
        max_count
    );
}

fn step2(
    input: &HashMap<[char; 2], i64>,
    insertions: &HashMap<[char; 2], Insertion>,
) -> HashMap<[char; 2], i64> {
    let mut result: HashMap<[char; 2], i64> = HashMap::new();

    for (k, v) in input {
        let left = k[0];
        let right = k[1];
        if let Some(rule) = insertions.get(k) {
            let new_left = [left, rule.element];
            let new_right = [rule.element, right];

            if let Some(existing) = result.insert(new_left, *v) {
                result.insert(new_left, v + existing);
            }
            if let Some(existing) = result.insert(new_right, *v) {
                result.insert(new_right, v + existing);
            }
        } else {
            result.insert(k.clone(), *v);
        }
    }

    result
}
