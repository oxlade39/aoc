fn main() {
    let input = include_str!("input.txt");
    let part1 = count_where_overlap(input);
    println!("part1: {part1}");
    let part2 = count_overlaps(input);
    println!("part2: {part2}");
}

fn count_where_overlap(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|items| {
                    let range_bounds: Vec<i32> = items
                        .split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    range_bounds[0]..range_bounds[1]
                })
                .collect::<Vec<_>>()
        })
        .map(|pair| {
            if pair[0].start >= pair[1].start && pair[0].end <= pair[1].end {
                return 1;
            }
            if pair[1].start >= pair[0].start && pair[1].end <= pair[0].end {
                return 1;
            }
            0
        })
        .sum()
}

fn count_overlaps(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|items| {
                    let range_bounds: Vec<i32> = items
                        .split("-")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    range_bounds[0]..range_bounds[1]
                })
                .collect::<Vec<_>>()
        })
        .map(|pair| {
            if pair[0].start <= pair[1].start && pair[0].end >= pair[1].start {
                return 1;
            }
            if pair[1].start <= pair[0].start && pair[1].end >= pair[0].start {
                return 1;
            }
            0
        })
        .sum()
}

#[test]
fn test_count_where_overlap_for_example() {
    let input = include_str!("input.example.txt");
    assert_eq!(2, count_where_overlap(input));
}

#[test]
fn test_count_overlaps() {
    let input = include_str!("input.example.txt");
    assert_eq!(4, count_overlaps(input));
}
