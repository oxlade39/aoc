fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let positions: Vec<_> = input
        .split(",")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    let max = (&positions).iter().max().unwrap().to_owned() as usize;
    println!("max: {}", max);

    let mut counts: Vec<i64> = vec![0; max];

    for i in 0..max {
        for pos in &positions {
            counts[i] += (pos - i as i64).abs();
        }
    }
    let min = (&counts).iter().min().unwrap();
    println!("min: {}", min);
}

fn part2() {
    let input = include_str!("input.txt");
    let positions: Vec<_> = input
        .split(",")
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    let max = (&positions).iter().max().unwrap().to_owned() as usize;
    println!("max: {}", max);

    let mut counts: Vec<i64> = vec![0; max];

    for i in 0..max {
        for pos in &positions {
            let abs_distance: i64 = (pos - i as i64).abs();
            let triangle_n = (abs_distance.pow(2) + abs_distance) / 2;
            counts[i] += triangle_n;
        }
    }
    let min = (&counts).iter().min().unwrap();
    println!("min: {}", min);
}
