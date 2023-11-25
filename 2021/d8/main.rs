use std::collections::{BTreeSet, HashMap, HashSet};

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
        let items = output
            .split(" ")
            .map(|item| item.len() as i32)
            .filter(|count| unique.contains(count))
            .count();
        count += items as i32;
    }

    println!("count: {}", count);
}

fn part2() {
    let input = include_str!("input.txt");

    let mut sum = 0;
    for line in input.lines() {
        sum += line_to_number(line);
    }
    println!("part2: {}", sum)
}

fn line_to_number(line: &str) -> i64 {
    let parts: Vec<_> = line.split(" | ").collect();
    let left = parts[0];
    let positions = build_positions(left);
    let numbers = create_mappings(positions);

    let right = parts[1];
    let right_chucks: Vec<_> = right
        .split(" ")
        .map(|str_item| {
            let letters: BTreeSet<char> = BTreeSet::from_iter(str_item.chars());
            letters
        })
        .collect();

    let mut output_n: i64 = 0;
    for (i, chunk) in right_chucks.iter().enumerate() {
        if let Some(it) = numbers.get(chunk) {
            let exp = right_chucks.len() - i - 1;
            let value = (10 as i64).pow(exp as u32) * *it as i64;
            output_n += value;
        }
    }
    output_n
}

fn create_mappings(positions: [char; 7]) -> HashMap<BTreeSet<char>, i32> {
    let mut layout_to_number: HashMap<BTreeSet<char>, i32> = HashMap::new();

    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[1],
            positions[2],
            positions[4],
            positions[5],
            positions[6],
        ]),
        0,
    );
    layout_to_number.insert(BTreeSet::from_iter([positions[2], positions[5]]), 1);

    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[2],
            positions[3],
            positions[4],
            positions[6],
        ]),
        2,
    );
    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[2],
            positions[3],
            positions[5],
            positions[6],
        ]),
        3,
    );
    layout_to_number.insert(
        BTreeSet::from_iter([positions[1], positions[2], positions[3], positions[5]]),
        4,
    );
    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[1],
            positions[3],
            positions[5],
            positions[6],
        ]),
        5,
    );
    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[1],
            positions[3],
            positions[4],
            positions[5],
            positions[6],
        ]),
        6,
    );
    layout_to_number.insert(
        BTreeSet::from_iter([positions[0], positions[2], positions[5]]),
        7,
    );
    layout_to_number.insert(BTreeSet::from_iter(positions), 8);
    layout_to_number.insert(
        BTreeSet::from_iter([
            positions[0],
            positions[1],
            positions[2],
            positions[3],
            positions[5],
            positions[6],
        ]),
        9,
    );

    return layout_to_number;
}

fn build_positions(left: &str) -> [char; 7] {
    let numbersa: Vec<_> = left
        .split(" ")
        .map(|str_item| {
            let letters: HashSet<char> = HashSet::from_iter(str_item.chars());
            letters
        })
        .collect();

    let mut known_positions: [char; 7] = ['-'; 7];
    let mut known_numbers: Vec<&HashSet<char>> = vec![&numbersa[0]; 9];

    for item in &numbersa {
        let item_len = item.len();
        match item_len {
            2 => known_numbers[1] = item,
            3 => known_numbers[7] = item,
            4 => known_numbers[4] = item,
            7 => known_numbers[8] = item,
            _ => (),
        }
    }

    known_positions[0] = *known_numbers[1]
        .symmetric_difference(&known_numbers[7])
        .next()
        .unwrap();

    let fives = intersect_all_of_length(5, &numbersa);
    known_positions[3] = *intersection_all(vec![&fives, known_numbers[4]])
        .iter()
        .next()
        .unwrap();

    let sixes = intersect_all_of_length(6, &numbersa);
    known_positions[5] = *intersection_all(vec![&sixes, known_numbers[1]])
        .iter()
        .next()
        .unwrap();

    let mut one = known_numbers[1].clone();
    one.remove(&known_positions[5]);
    known_positions[2] = *one.iter().next().unwrap();

    let mut four = known_numbers[4].clone();
    four.remove(&known_positions[2]);
    four.remove(&known_positions[3]);
    four.remove(&known_positions[5]);
    known_positions[1] = *four.iter().next().unwrap();

    let mut fives_cp = fives.clone();
    fives_cp.remove(&known_positions[0]);
    fives_cp.remove(&known_positions[3]);
    known_positions[6] = *fives_cp.iter().next().unwrap();

    let mut eight = known_numbers[8].clone();
    for pos in known_positions {
        eight.remove(&pos);
    }
    known_positions[4] = *eight.iter().next().unwrap();

    println!("");
    println!("  {} ", known_positions[0]);
    println!("{}  {}", known_positions[1], known_positions[2]);
    println!("  {} ", known_positions[3]);
    println!("{}  {}", known_positions[4], known_positions[5]);
    println!("  {} ", known_positions[6]);
    println!("");

    return known_positions;
}

fn intersect_all_of_length(n: usize, values: &Vec<HashSet<char>>) -> HashSet<char> {
    let selected: Vec<_> = values.iter().filter(|item| item.len() == n).collect();
    intersection_all(selected)
}

fn intersection_all(sets: Vec<&HashSet<char>>) -> HashSet<char> {
    let mut first: HashSet<char> = sets[0].clone();
    for next in sets.iter().skip(1) {
        first = first.intersection(*next).map(|c| *c).collect();
    }

    first
}
