use std::{str::FromStr, collections::{HashSet, HashMap}};

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

    println!("insertions: {:?}", insertions);


    // let mut result: Vec<char> = Vec::new();
    // result.push(chain.chars().nth(0).unwrap());
    // for chunk in chain.chars().collect::<Vec<char>>().windows(2) {        
    //     if let Some(insertion) = insertions.get(chunk) {
    //         let next = insertion.insert();
    //         result.push(next[1]);
    //         result.push(next[2]);
    //     }
    // }
    let mut input: Vec<_> = chain.chars().collect();
    let n = 10;
    for _ in 0..n {
        input = step(&input, &insertions);
    }

    let mut counts: HashMap<char, i64> = HashMap::new();

    for c in &input {
        let i = counts.get(&c).unwrap_or(&0);
        counts.insert(c.clone(), i+1);
    }

    let (max, max_count) = counts.iter()
        .max_by(|left, right| left.1.cmp(right.1))
        .unwrap();

    let (min, min_count) = counts.iter()
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
    element: char
}

impl Insertion {
    fn insert(&self) -> [char; 3]{
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
        Ok(Insertion{ chain: [c[0], c[1]], element })
    }
}


fn part2() {

}