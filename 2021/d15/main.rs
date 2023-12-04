use std::collections::BTreeSet;

use std::time::Instant;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

/// Use sufficiently high number that a real hueristic wouldn't be above
const INFINITY: i64 = 1000000;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {}ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");
    let grid: Grid = input.parse().unwrap();
    let height: i64 = grid.1 as i64;
    let width: i64 = grid.2 as i64;
    let result = astar(&grid, (0, 0), (height - 1, width - 1));
    println!("result: {}", result);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut grid: Grid = input.parse().unwrap();
    grid = expand(&grid, 5);
    let height: i64 = grid.1 as i64;
    let width: i64 = grid.2 as i64;

    let result = astar(&grid, (0, 0), (height - 1, width - 1));
    println!("result: {}", result);
}

#[derive(Clone, Debug)]
struct Candidate((i64, i64), i64);

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// Based on https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode
fn astar(grid: &Grid, start: (i64, i64), end: (i64, i64)) -> i64 {
    let mut open_set: BTreeSet<Candidate> = BTreeSet::new();
    let mut came_from: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    let mut g_scores: HashMap<(i64, i64), i64> = HashMap::new();
    let mut f_scores: HashMap<(i64, i64), i64> = HashMap::new();

    open_set.insert(Candidate(start, (grid.1 * grid.2) as i64));
    g_scores.insert(start, 0);
    f_scores.insert(start, (grid.1 * grid.2) as i64);

    loop {
        if open_set.is_empty() {
            break;
        }

        // this is slow, optimise to use a priority queue
        let curr_candid = open_set.first().unwrap().clone();
        let curr_node = curr_candid.0;

        if curr_node.0 == end.0 && curr_node.1 == end.1 {
            let mut path_node = Some(curr_node);
            let mut sum = 0;
            loop {
                if let Some(x) = path_node {
                    let cost = grid.0[x.0 as usize][x.1 as usize];
                    sum += cost;
                    path_node = came_from.get(&x).map(|item| *item);
                } else {
                    return sum - grid.0[start.0 as usize][start.1 as usize];
                }
            }
        }

        open_set.remove(&curr_candid);

        for neighbour_coord in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let neighbour = (
                curr_node.0 + neighbour_coord.0,
                curr_node.1 + neighbour_coord.1,
            );
            if neighbour.0 < 0
                || neighbour.0 >= grid.1 as i64
                || neighbour.1 < 0
                || neighbour.1 >= grid.2 as i64
            {
                continue;
            }

            let neighbour_row = &grid.0[neighbour.0 as usize];
            let neighbour_cost = neighbour_row[neighbour.1 as usize];
            let neighbour_g_score = g_scores.get(&neighbour).unwrap_or(&INFINITY);
            let tentative_g_score = g_scores.get(&curr_node).unwrap_or(&INFINITY) + neighbour_cost;
            if tentative_g_score < *neighbour_g_score {
                came_from.insert(neighbour, curr_node);
                g_scores.insert(neighbour, tentative_g_score);

                // distance to target
                let hueristic = i64::abs(end.0 - neighbour.0) + i64::abs(end.1 - neighbour.1);
                f_scores.insert(neighbour, tentative_g_score + hueristic);
                open_set.insert(Candidate(neighbour, tentative_g_score + hueristic));
            }
        }
    }

    return -1;
}

struct Grid(Vec<Vec<i64>>, usize, usize);

#[derive(Debug, PartialEq, Eq)]
struct GridParseErr {
    line: usize,
    col: usize,
    value: char,
}

impl FromStr for Grid {
    type Err = GridParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const RADIX: u32 = 10;

        let mut grid: Vec<Vec<i64>> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                if let Some(value) = c.to_digit(RADIX) {
                    row.push(value as i64);
                } else {
                    return Err(GridParseErr {
                        line: i,
                        col: j,
                        value: c,
                    });
                }
            }
            grid.push(row);
        }
        let rows = grid.len();
        let cols = grid[0].len();
        Ok(Grid(grid, rows, cols))
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            writeln!(f, "{:?}", v)?;
        }
        Ok(())
    }
}

fn expand(grid: &Grid, times: usize) -> Grid {
    let new_rows = grid.1 * times;
    let new_cols = grid.2 * times;
    let mut result: Vec<Vec<i64>> = Vec::with_capacity(new_rows);
    println!(
        "expanding grid to h:{} w:{} from {},{}",
        new_rows, new_cols, grid.1, grid.2
    );

    for row in 0..new_rows {
        let mut row_vals: Vec<i64> = Vec::with_capacity(new_cols);

        for col in 0..new_cols {
            if row < grid.1 && col < grid.2 {
                row_vals.push(grid.0[row][col]);
            } else {
                if col < grid.2 {
                    let v = result[row - grid.1][col] + 1;
                    let mut wrap_around = v % 10;
                    if wrap_around == 0 {
                        wrap_around = 1;
                    }
                    row_vals.push(wrap_around);
                } else {
                    let v = row_vals[col - grid.2] + 1;
                    let mut wrap_around = v % 10;
                    if wrap_around == 0 {
                        wrap_around = 1;
                    }
                    row_vals.push(wrap_around);
                }
            }
        }
        result.push(row_vals);
    }

    Grid(result, grid.1 * times, grid.2 * times)
}

#[test]
fn test_expand_example() {
    let input = include_str!("input.test.txt");
    let grid: Grid = input.parse().unwrap();

    let input = include_str!("input.test.expanded.txt");
    let expanded_grid: Grid = input.parse().unwrap();

    assert_eq!(expand(&grid, 5).0, expanded_grid.0);
}
