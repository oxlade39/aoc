use std::collections::{HashMap, HashSet};


fn main() {
    let input = include_str!("input.txt");
    let visible = resolve_visible(input);
    println!("part1: {}", visible.len());
}

#[test]
fn test_pt1_example() {
    let input = include_str!("input.example.txt");
    let visible = resolve_visible(input);
    print_visible(&visible, 5, 5);
    assert_eq!(21, visible.len(), "visible: {:?}", visible);
}

fn print_visible(visible: &HashSet<(usize, usize)>, width: usize, height: usize) {
    for i in 0..=height {
        println!("");
        for j in 0..=width {
            if visible.contains(&(i, j)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
    }
}

fn resolve_visible(input: &str) -> HashSet<(usize, usize)> {
    
    let heights = input
        .lines()
        .map(|line| line
            .chars()
            .map(|digit| digit.to_digit(10).expect("digit") as i32)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // left to right
    for height in 0..heights.len() {
        let mut max: i32 = -1;
        for width in 0..heights[0].len() {
            let item = heights[height][width];
            if item > max {
                max = item;
                visible.insert((height, width));
            }            
        }
    }

    // right to left
    for height in 0..heights.len() {
        let mut max = -1;
        for width in (0..heights[0].len()).rev() {
            let item = heights[height][width];
            if item > max {
                max = item;
                visible.insert((height, width));
            }            
        }
    }

    // top to bottom
    for width in 0..heights[0].len() {
        let mut max = -1;
        for height in 0..heights.len() {
            let item = heights[height][width];
            if item > max {
                max = item;
                visible.insert((height, width));
            }            
        }
    }

    // bottom to top
    for width in 0..heights[0].len() {
        let mut max = -1;
        for height in (0..heights.len()).rev() {
            let item = heights[height][width];
            if item > max {
                max = item;
                visible.insert((height, width));
            }            
        }
    }
    visible
}