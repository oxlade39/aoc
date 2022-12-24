use std::collections::HashMap;


fn main() {
    let input = include_str!("input.txt");
    let solver = Day6 { distinct_count: 4 };
    let pt1 = solver.solve(input);
    println!("pt1: {}", pt1);

    let solver = Day6 { distinct_count: 14 };
    let pt2 = solver.solve(input);
    println!("pt2: {}", pt2);
}

trait Solver {
    fn solve(&self, datastream: &str) -> usize;
}

struct Day6 { distinct_count: usize }

impl Solver for Day6 {
    fn solve(&self, datastream: &str) -> usize {
        let chars: Vec<_> = datastream.chars().collect();
        let mut window: HashMap<char, usize> = HashMap::new();

        for i in 0..chars.len() {
            // println!("- i: {:?} - current: {:?} - window: {:?}", i, chars[i], window);
            if i > self.distinct_count - 1 {
                if let Some(count) = window.remove(&chars[i - self.distinct_count]) {
                    if count > 1 {
                        window.insert(chars[i - self.distinct_count], count - 1);
                    }                    
                }
                if let Some(count) = window.insert(chars[i], 1) {
                    window.insert(chars[i], count + 1);
                }
                if window.len() == self.distinct_count {
                    return i + 1;
                }
            } else {
                if let Some(count) = window.insert(chars[i], 1) {
                    window.insert(chars[i], count + 1);
                }
            }
            // println!("+ i: {:?} - current: {:?} - window: {:?}", i, chars[i], window);
        }
        
        panic!("not found");
    }
}

#[test]
fn test_part1_examples() {
    let examples = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, "Z"),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, "A"),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, "B"),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, "C"),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, "D"),
    ];

    let solver = Day6 { distinct_count: 4 };
    for (ex_in, ex_out, name) in examples {
        let out = solver.solve(ex_in);
        assert_eq!(ex_out, out, "example: {}", name);
    }
}

#[test]
fn test_part2_examples() {
    let examples = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19, "A"),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23, "B"),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23, "C"),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29, "D"),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26, "D"),
    ];

    let solver = Day6 { distinct_count: 14 };
    for (ex_in, ex_out, name) in examples {
        let out = solver.solve(ex_in);
        assert_eq!(ex_out, out, "example: {}", name);
    }
}