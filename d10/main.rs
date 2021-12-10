use std::collections::HashMap;

fn main() {
    part1();
}

fn part1() {
    let input = include_str!("input.txt");

    let opens = brackets();
    let points = part1_points();

    let all: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut total_points = 0;

    let mut incomplete: Vec<Vec<char>> = Vec::new();

    for line in all {
        let mut items: Vec<char> = Vec::new();
        for (i, next) in line.iter().enumerate() {
            if let Some(close) = opens.get(&next) {
                items.push(*close);
            } else {
                if next == items.last().unwrap() {
                    items.pop().unwrap();
                } else {
                    // must be bad
                    let p = points.get(&next).unwrap();
                    total_points += p;
                    break;
                }
            }
            if i == line.len() - 1 {
                incomplete.push(items.clone());
            }
        }
    }

    println!("part 1: {}", total_points);
    part2(&incomplete);
}

fn brackets() -> HashMap<char, char> {
    [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect()
}

fn part1_points() -> HashMap<char, i64> {
    [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect()
}

fn part2_points() -> HashMap<char, i64> {
    [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect()
}

fn part2(incomplete: &Vec<Vec<char>>) {
    let pt2_points = part2_points();
    let mut part2: Vec<i64> = Vec::new();
    for it in incomplete {
        let mut line_total = 0;
        for c in it.iter().rev() {
            line_total = line_total * 5;
            let msg = format!("expected for {}", c);
            line_total += pt2_points.get(c).expect(&msg);
        }
        part2.push(line_total);
    }

    part2.sort();
    println!("part 2: {:?}", part2[part2.len() / 2]);
}
