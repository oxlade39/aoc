use std::{time::Instant, str::FromStr};

use aoclib::input;
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    input::empty_line_chunks(txt)
        .map(|item| item.parse::<Grid>().unwrap())
        .map(|g| {
            let col = g.symmetric_col();
            let row = g.symmetric_row();

            col + 100 * row
        })
        .sum()
}

fn part2(txt: &str) -> usize {
    input::empty_line_chunks(txt)
        .map(|item| item.parse::<Grid>().unwrap())
        .map(|g| g.calc())
        .sum()
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn at(&self, row: usize, col: usize) -> char {
        self.0[row][col]
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn calc(mut self) -> usize {
        let row_bits = self.row_as_bits();
        if let Some((row, fix)) = symmetric(row_bits, self.width(), None) {
            let row_score = 100 * row;

            if let Some((fix_row, fix_col)) = fix {
                if self.0[fix_row][fix_col] == '#' {
                    println!("flipping # to .\n");
                    self.0[fix_row][fix_col] = '.';
                } else {
                    println!("flipping . to #\n");
                    self.0[fix_row][fix_col] = '#';
                }
            }

            println!("fixed....");
            for i in 0..self.height() {
                for j in 0..self.width() {
                    print!("{}", self.0[i][j]);
                }
                println!("")
            }

            let col_bits = self.col_as_bits();
            let col_score = symmetric(col_bits, self.height(), fix)
                .map(|(col_num, _)| col_num)
                .unwrap_or(0);

            row_score + col_score
        } else {
            let col_bits = self.col_as_bits();
            symmetric(col_bits, self.height(), None)
                .map(|(col_num, _)| col_num)
                .unwrap_or(0)
        }
    }

    fn symmetric_col(&self) -> usize {
        for col in 0..(self.width() - 1) {
            let mut symmetric = true;

            for offset in 0..self.width() {
                if !symmetric {
                    continue;
                }

                let forward_col_index = col + offset + 1;
                let back_col_index = col as i32 - offset as i32;

                if back_col_index < 0 || forward_col_index >= self.width() {
                    break;
                }

                for row in 0..self.height() {
                    let forward = self.at(row, forward_col_index);
                    let backward = self.at(row, back_col_index as usize);
    
                    if forward != backward {
                        symmetric = false;
                        break;
                    }
                }                
            }
            if symmetric {
                return col + 1;
            }
        }
        return 0;
    }

    fn symmetric_row(&self) -> usize {
        for row in 0..(self.height() - 1) {
            let mut symmetric = true;

            for offset in 0..self.height() {
                let forward_row_index = row + offset + 1;
                let back_row_index = row as i32 - offset as i32;

                if back_row_index < 0 || forward_row_index >= self.height() {
                    break;
                }

                if !symmetric {
                    break;
                }

                for col in 0..self.width() {
                    let forward = self.at(forward_row_index, col);
                    let backward = self.at(back_row_index as usize, col);

                    if forward != backward {
                        symmetric = false;
                        break;
                    }
                }                
            }

            if symmetric {
                return row + 1;
            }
        }
        return 0;
    }

    fn row_as_bits(&self) -> Vec<i32> {
        let grid = self;
        (0..grid.height())
                .map(|row_num| {
                    (0..grid.width())
                        .fold(0_i32, |accum, col_num| {
                            let c = grid.at(row_num, col_num);
                            let bit_num = grid.0[0].len() - col_num - 1;
                            if c == '#' {                     
                                accum | 1 << bit_num
                            } else {
                                accum
                            }                            
                        })
                })
                .collect()
    }

    fn col_as_bits(&self) -> Vec<i32> {
        let grid = self;
        (0..grid.width())
            .map(|col_num| {
                (0..grid.height())
                    .fold(0_i32, |accum, row_num| {
                        let c = grid.at(row_num, col_num);
                        let bit_num = grid.0.len() - row_num - 1;
                        if c == '#' {                     
                            accum | 1 << bit_num
                        } else {
                            accum
                        }
                    })
            })
            .collect()
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(s.lines().map(|l| l.chars().collect_vec()).collect_vec()))
    }
}

fn symmetric(items: Vec<i32>, width: usize, fix: Option<(usize, usize)>) -> Option<(usize, Option<(usize, usize)>)> {    
    let len: i32 = items.len() as i32;

    items.windows(2)
        .enumerate()
        .filter_map(|(i, window)| {
            let left = window[0];
            let right = window[1];

            if left == right {
                println!("symmetry at {i}");
                return Some((i, fix));
            }

            if fix.is_none() {
                // or try flipping a bit
                let xor = left ^ right;
                let left_xor_or = left | xor;
                let right_xor_or = right | xor;

                if left_xor_or == left || right_xor_or == right {

                    if left != left_xor_or {
                        println!("fixed symmetry at {i} with {xor:b} and {}", (xor as f32).log2());
                        let fix = width - 1 - (xor as f32).log2().round() as usize;                    
                        return Some((i, Some((i, fix))))
                    } else {
                        println!("fixed symmetry at {i} with {xor:b} and {}", (xor as f32).log2());
                        let fix = width - 1 - (xor as f32).log2().round() as usize;                    
                        return Some((i, Some((i + 1, fix))))
                    }
                    
                }
            }            

            None
        })
        .find_map(|(i, fixed)| {

            let mut offset = 1;

            let mut fix: Option<(usize, usize)> = fixed;

            let mut left_pos: i32 = i as i32 - offset;
            let mut right_pos: i32 = i as i32 + offset + 1;
            while left_pos >= 0 && right_pos < len {
                let left = items[left_pos as usize];
                let right = items[right_pos as usize];                
                if left != right {
                    println!("{left:b} vs {right:b} at {left_pos} vs {right_pos}");

                    if fixed.is_none() {
                        // or try flipping a bit
                        let xor = left ^ right;
                        let left_xor_or = left | xor;
                        let right_xor_or = right | xor;

                        if left_xor_or == left {
                            println!("fixed left at {left_pos} with {xor:b} {} - {}", width, (xor as f32).log2());
                            fix = Some((left_pos as usize, width - 1 - (xor as f32).log2().round() as usize));
                        } else if right_xor_or == right {
                            println!("fixed right at {right_pos} with {xor:b} {}", (xor as f32).log2());
                            fix = Some((right_pos as usize, width - 1 - (xor as f32).log2().round() as usize));
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }                    
                }
                offset += 1;
                left_pos = i as i32 - offset;
                right_pos = i as i32 + offset + 1;
            }

            Some((i + 1, fix))
        })
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(405, part1(include_str!("input.test.txt")));
    }


    // #[test]
    // fn test_example_p2() {
    //     assert_eq!(400, part2(include_str!("input.test.txt")));
    // }    

    #[test]
    fn test_symmetric_col() {
        let grids = input::empty_line_chunks(include_str!("input.test.txt"))
            .map(|chunk| chunk.parse::<Grid>().unwrap())
            .collect_vec();        
        assert_eq!(5, grids[0].symmetric_col());
        assert_eq!(0, grids[0].symmetric_row());
    }
    
    #[test]
    fn test_symmetric_row() {
        let grids = input::empty_line_chunks(include_str!("input.test.txt"))
            .map(|chunk| chunk.parse::<Grid>().unwrap())
            .collect_vec();        
        assert_eq!(4, grids[1].symmetric_row());
        assert_eq!(0, grids[1].symmetric_col());

    }

    #[test]
    fn test_row_as_bits() {
        let grids = input::empty_line_chunks(include_str!("input.test.txt"))
            .map(|chunk| chunk.parse::<Grid>().unwrap())
            .collect_vec();

        let bits: Vec<String> = grids[0].row_as_bits().iter()
            .map(|&row| format!("{row:b}"))
            .collect();

        assert_eq!("101100110", bits[0]);

        let bits: Vec<String> = grids[1].row_as_bits().iter()
            .map(|&row| format!("{row:b}"))
            .collect();

        assert_eq!("100011001", bits[0]);
    }

    #[test]
    fn test_col_as_bits() {
        let grids = input::empty_line_chunks(include_str!("input.test.txt"))
            .map(|chunk| chunk.parse::<Grid>().unwrap())
            .collect_vec();

        let bits: Vec<String> = grids[0].col_as_bits().iter()
            .map(|&row| format!("{row:b}"))
            .collect();

        assert_eq!("1011001", bits[0]);

        let bits: Vec<String> = grids[1].col_as_bits().iter()
            .map(|&row| format!("{row:b}"))
            .collect();

        assert_eq!("1101101", bits[0]);
    }

    #[test]
    fn test_vec_symmetric() {
        let grids: Vec<_> = input::empty_line_chunks(include_str!("input.test.txt"))
            .map(|chunk| chunk.parse::<Grid>().unwrap())
            .collect();
        
        let grid = grids[0].clone();
        let result = symmetric(grid.row_as_bits(), grid.0[0].len(), None);
        assert_eq!(Some((3, Some((0, 0)))), result);

        let grid = grids[1].clone();
        let result = symmetric(grid.row_as_bits(), grid.0[0].len(), None);
        assert_eq!(Some((1, Some((1, 4)))), result);

        // let grids: Vec<_> = input::empty_line_chunks(include_str!("input.test.txt"))
        //     .map(|chunk| chunk.parse::<Grid>().unwrap())
        //     .map(|g| g.col_as_bits())
        //     .collect();

        // let result = symmetric(grids[0].clone());
        // assert_eq!(None, result);

        // let result = symmetric(grids[1].clone());
        // assert_eq!(None, result);
    }

    // #[test]
    // fn test_part2() {
    //     // let result = part2(include_str!("input.test.txt"));
    //     // assert_eq!(400, result);
    //     let grid = input::empty_line_chunks(include_str!("input.test.txt"))
    //         .map(|l| l.parse::<Grid>().unwrap())
    //         .collect_vec();
        
    //     assert_eq!(300, grid[0].clone().calc());
    //     // assert_eq!(100, grid[1]);
    // }
}
