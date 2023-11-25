use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");

    let items: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split("")
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let height = items.len();
    let width = items[0].len();

    let mut sum = 0;

    for (row, line) in items.iter().enumerate() {
        for (col, n) in line.iter().enumerate() {
            let mut neighbours: Vec<i64> = Vec::new();

            if row > 0 {
                neighbours.push(items[row - 1][col]);
            }
            if row + 1 < height {
                neighbours.push(items[row + 1][col]);
            }
            if col + 1 < width {
                neighbours.push(items[row][col + 1]);
            }
            if col > 0 {
                neighbours.push(items[row][col - 1]);
            }

            let min_neighbour = neighbours.iter().min().unwrap();
            if n < min_neighbour {
                sum += 1 + n;
            }
        }
    }

    println!("sum: {:?}", sum)
}

fn part2() {
    let input = include_str!("input.txt");

    let items: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split("")
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let height = items.len();
    let width = items[0].len();

    let mut basins: Vec<HashSet<Postion>> = Vec::new();

    for (row, line) in items.iter().enumerate() {
        for (col, n) in line.iter().enumerate() {
            let mut neighbours: Vec<i64> = Vec::new();

            if row > 0 {
                neighbours.push(items[row - 1][col]);
            }
            if row + 1 < height {
                neighbours.push(items[row + 1][col]);
            }
            if col + 1 < width {
                neighbours.push(items[row][col + 1]);
            }
            if col > 0 {
                neighbours.push(items[row][col - 1]);
            }

            let min_neighbour = neighbours.iter().min().unwrap();
            if n < min_neighbour {
                let bs = basin_size(width, height, Postion { row, col }, &items, &HashSet::new());
                basins.push(bs);
            }
        }
    }
    let mut basin_sizes: Vec<_> = basins.iter().map(|p| p.len()).collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    let top_three_product: i64 = basin_sizes.iter().take(3).map(|i| *i as i64).product();
    println!("basins: {:?}", top_three_product);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Postion {
    row: usize,
    col: usize,
}

fn basin_size(
    width: usize,
    height: usize,
    position: Postion,
    grid: &Vec<Vec<i64>>,
    lows: &HashSet<Postion>,
) -> HashSet<Postion> {
    let mut return_set = HashSet::new();
    return_set.extend(lows);

    let row = position.row;
    let col = position.col;

    if grid[row][col] == 9 {
        return return_set;
    }

    if return_set.contains(&position) {
        return return_set;
    }

    return_set.insert(position);

    if row > 0 {
        let up = basin_size(
            width,
            height,
            Postion { row: row - 1, col },
            grid,
            &return_set,
        );
        return_set.extend(up);
    }

    if col > 0 {
        let left = basin_size(
            width,
            height,
            Postion { row, col: col - 1 },
            grid,
            &return_set,
        );
        return_set.extend(left);
    }

    if row < height - 1 {
        let down = basin_size(
            width,
            height,
            Postion { row: row + 1, col },
            grid,
            &return_set,
        );
        return_set.extend(down);
    }

    if col < width - 1 {
        let right = basin_size(
            width,
            height,
            Postion { row, col: col + 1 },
            grid,
            &return_set,
        );
        return_set.extend(right);
    }

    return_set
}
