use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input.txt");
    let result = max(input);
    println!("max: {result}");
    let result_n = sum_max_n(input, 3);
    println!("max n: {result_n}");
}

fn max(input: &str) -> i32 {
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if let Ok(i) = line.parse::<i32>() {
            current += i;
        } else {
            if current > max {
                max = current;
            }
            current = 0;
        }
    }
    max
}

fn sum_max_n(input: &str, n: usize) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut current = 0;

    for line in input.lines() {
        if let Ok(i) = line.parse::<i32>() {
            current += i;
        } else {
            heap.push(current);
            current = 0;
        }
    }
    heap.push(current);

    let mut count = n;
    let mut sum = 0;
    while count > 0 {
        if let Some(i) = heap.pop() {
            println!("got: {i}");
            sum += i;
        }
        count -= 1;
    }
    sum
}

#[test]
fn test_example() {
    let input = include_str!("input.test.txt");
    assert_eq!(24000, max(input));
}

#[test]
fn test_max_n_example() {
    let input = include_str!("input.test.txt");
    assert_eq!(45000, sum_max_n(input, 3));
}
