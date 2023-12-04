use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let visible = resolve_visible(input);
    println!("part1: {}", visible.len());

    let heights = parse_heights(input);
    let scores = build_scenic_score_grid(&heights);
    let max_score = scores.iter().flat_map(|row| row).max().unwrap();
    println!("part2: {}", max_score);
}

#[test]
fn test_pt1_example() {
    let input = include_str!("input.example.txt");
    let visible = resolve_visible(input);
    print_visible(&visible, 5, 5);
    assert_eq!(21, visible.len(), "visible: {:?}", visible);
}

#[test]
fn test_pt2_example() {
    let input = include_str!("input.example.txt");
    let heights = parse_heights(input);

    println!("\nheights");

    for height_row in &heights {
        for height_col in height_row {
            print!(" {} ", height_col);
        }
        println!("");
    }

    let scores = build_scenic_score_grid(&heights);

    println!("\nscores {}x{}", scores.len(), scores[0].len());

    for y in 0..scores.len() {
        for x in 0..scores[0].len() {
            print!(" {} ", scores[y][x]);
        }
        println!("");
    }

    assert_eq!(4, scores[1][2], "1,2");
    assert_eq!(8, scores[3][2], "3,2");
}

#[cfg(test)]
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

fn parse_heights(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).expect("digit") as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn resolve_visible(input: &str) -> HashSet<(usize, usize)> {
    let heights = parse_heights(input);

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

fn build_scenic_score_grid(heights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let max_height = heights.len() - 1;
    let max_width = heights[0].len() - 1;

    let mut scenic_scores: Vec<Vec<i32>> = vec![vec![1; heights[0].len()]; heights.len()];

    // zeros at edges - top/bottom
    for x in 0..=max_width {
        scenic_scores[0][x] = 0;
        scenic_scores[max_height][x] = 0;
    }

    // zeros at edges - left/right
    for y in 0..=max_height {
        scenic_scores[y][0] = 0;
        scenic_scores[y][max_width] = 0;
    }

    for height in 1..=(max_height - 1) {
        for width in 1..=(max_width - 1) {
            let current_height = heights[height][width];

            let mut tree_counts = [0, 0, 0, 0];

            // look right
            for x in (width + 1)..=max_width {
                if heights[height][x] < current_height {
                    tree_counts[0] += 1;
                } else {
                    tree_counts[0] += 1;
                    break;
                }
            }

            // look left
            for x in (0..=(width - 1)).rev() {
                if heights[height][x] < current_height {
                    tree_counts[1] += 1;
                } else {
                    tree_counts[1] += 1;
                    break;
                }
            }

            // look up
            for y in (0..=(height - 1)).rev() {
                if heights[y][width] < current_height {
                    tree_counts[2] += 1;
                } else {
                    tree_counts[2] += 1;
                    break;
                }
            }

            // look down
            for y in (height + 1)..=max_height {
                if heights[y][width] < current_height {
                    tree_counts[3] += 1;
                } else {
                    tree_counts[3] += 1;
                    break;
                }
            }

            scenic_scores[height][width] =
                tree_counts[0] * tree_counts[1] * tree_counts[2] * tree_counts[3];
        }
    }

    scenic_scores
}
