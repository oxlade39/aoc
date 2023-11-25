fn main() {
    let input = include_str!("input.txt");
    let mut counts: [i64; 9] = [0; 9];

    for n in input.split(",").map(|c| c.parse::<usize>().unwrap()) {
        counts[n] += 1;
    }
    let day = 256;

    for _ in 0..day {
        let carry = counts[0];
        for bucket in 0..8 {
            counts[bucket] = counts[bucket + 1];
        }
        counts[6] += carry;
        counts[8] = carry;
    }
    let answer: i64 = counts.iter().sum();
    println!("answer: {}", answer)
}
