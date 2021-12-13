use std::{str::FromStr, collections::HashSet};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.test.txt");
    
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    parse(input, &mut positions, &mut folds);    

    print_grid(&positions);

    let result = folds.iter()
        .take(1)
        .fold(positions.clone(), |accum, item| fold(accum, item));

    print_grid(&result);

    let result2 = folds.iter()
        .take(2)
        .fold(positions.clone(), |accum, item| fold(accum, item));

    print_grid(&result2);
}

fn print_grid(result: &HashSet<(i64, i64)>) {
    let max_x = result.iter().map(|(x,_y)| *x).max().unwrap();
    let max_y = result.iter().map(|(_x,y)| *y).max().unwrap();

    println!("");
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            let pos = (x, y);
            if result.contains(&pos) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!("")
    }
    println!("");
}

fn fold(positions: HashSet<(i64, i64)>, fold: &Fold) -> HashSet<(i64, i64)> {
    match fold {
        Fold::AlongY(pos) => fold_y(positions, *pos),
        Fold::AlongX(pos) => fold_x(positions, *pos)
    }
}

fn fold_y(positions: HashSet<(i64, i64)>, fold: i64) -> HashSet<(i64, i64)> {
    let mut result: HashSet<(i64, i64)> = HashSet::new();

    for no_move in positions.iter().filter(|(_x, y)| *y < fold) {
        result.insert(no_move.clone());
    }

    for (x, y) in positions.iter().filter(|(_x, y)| *y > fold) {
        let new_x = *x;
        let n = y - fold;
        let new_y = fold - n;
        if new_y < fold {
            result.insert((new_x, new_y));
        }        
    }

    result
}

fn fold_x(positions: HashSet<(i64, i64)>, fold: i64) -> HashSet<(i64, i64)> {
    let mut result: HashSet<(i64, i64)> = HashSet::new();

    for no_move in positions.iter().filter(|(x, _y)| *x < fold) {
        result.insert(no_move.clone());
    }

    for (x, y) in positions.iter().filter(|(x, _y)| *x > fold) {
        let new_y = *y;
        let n = x - fold;
        let new_x = fold - n;
        if new_x < fold {
            result.insert((new_x, new_y));
        }        
    }

    result
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Fold {
    AlongX(i64),
    AlongY(i64)
}

#[derive(Debug)]
struct InputErr;

impl FromStr for Fold {
    type Err = InputErr;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = line.split("fold along ").collect();
        let axis_quantity: Vec<_> = parts[1].split("=").collect();
        match axis_quantity[0] {
            "y" => Ok(Fold::AlongY(axis_quantity[1].parse().unwrap())),
            "x" => Ok(Fold::AlongX(axis_quantity[1].parse().unwrap())),
            _ => Err(InputErr)
        }
    }
}

#[test]
fn test_fold_y(){
    let input = HashSet::from_iter(vec![
        (0,4), (1, 5), (2, 6)
    ]);
    let result = fold_y(input, 3);

    println!("fold y=3 -> {:?}", result);
}

fn part2() {
    let input = include_str!("input.txt");
    
    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    let mut folds: Vec<Fold> = Vec::new();

    parse(input, &mut positions, &mut folds);

    let result = folds.iter()
        .fold(positions, |accum, item| fold(accum, item));
    
    print_grid(&result);
}

fn parse(input: &str, positions: &mut HashSet<(i64, i64)>, folds: &mut Vec<Fold>) {
    for line in input.lines() {
        if line.len() < 3 {
            continue;
        }

        if line.chars().nth(0).unwrap() != 'f' {
            let parts: Vec<_> = line.split(",").collect();
            let x: i64 = parts[0].parse().unwrap();
            let y: i64 = parts[1].parse().unwrap();
            positions.insert((x, y));
        } else {
            let fold: Fold = line.parse().unwrap();
            folds.push(fold);
        }
    }
}