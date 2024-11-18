use std::{collections::HashSet, fmt::Debug, str::FromStr};

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("input.txt");
    let mut grid: Grid = input.parse().unwrap();

    let steps = 100;
    let mut flash_count = 0;

    for step in 0..steps {
        let flashes = grid.step();
        flash_count += flashes.len();

        println!("Step {}", step + 1);
        println!("{:?}", grid);
        println!("");
    }
    println!("part1: {}", flash_count);
}

fn part2() {
    let input = include_str!("input.txt");
    let mut grid: Grid = input.parse().unwrap();

    let mut step = 0;

    loop {
        let flashed = grid.step();
        step += 1;
        let num_flashed = flashed.len();
        if num_flashed == 100 {
            println!("part2: {}", step);
            break;
        }
    }
}

fn neighbours(row: i32, col: i32) -> Vec<(i32, i32)> {
    let all = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];
    all.iter()
        .filter(|item| {
            let (y, x) = item;
            !(*y < 0 || *x < 0 || *y > 9 || *x > 9)
        })
        .map(|item| *item)
        .collect()
}

struct Grid(Vec<Vec<i32>>);

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

        let mut grid: Vec<Vec<i32>> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if let Some(value) = c.to_digit(RADIX) {
                    grid[i][j] = value as i32;
                } else {
                    return Err(GridParseErr {
                        line: i,
                        col: j,
                        value: c,
                    });
                }
            }
        }
        Ok(Grid(grid))
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

impl Grid {
    fn step(&mut self) -> HashSet<(i32, i32)> {
        let mut to_flash: HashSet<(i32, i32)> = HashSet::new();
        for i in 0..10 {
            for j in 0..10 {
                self.0[i][j] += 1;

                if self.0[i][j] > 9 {
                    to_flash.insert((i as i32, j as i32));
                }
            }
        }
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        loop {
            if let Some(item) = to_flash.clone().iter().next() {
                let row = item.0;
                let col = item.1;
                to_flash.remove(item);

                if flashed.insert(item.clone()) {
                    let neighbours = neighbours(row, col);

                    for n in neighbours {
                        self.0[n.0 as usize][n.1 as usize] += 1;
                        if self.0[n.0 as usize][n.1 as usize] > 9 {
                            to_flash.insert(n);
                        }
                    }
                }
            } else {
                break;
            }
        }

        for flasher in &flashed {
            self.0[flasher.0 as usize][flasher.1 as usize] = 0;
        }

        flashed
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    row: i32,
    col: i32,
}

impl From<(i32, i32)> for Position {
    fn from(pair: (i32, i32)) -> Self {
        Position {
            row: pair.0,
            col: pair.1,
        }
    }
}
