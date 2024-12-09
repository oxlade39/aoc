use core::str;
use std::{time::Instant, usize};

use aoclib::timing;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let mut disk: Vec<Block> = Vec::new();
    for (i, c) in txt.chars().enumerate() {
        let block =  if i % 2 == 0 {
            Block::File(i / 2)
        } else {
            Block::Space    
        };
        let size = c.to_digit(10).unwrap();
        disk.extend(std::iter::repeat_n(block, size as usize));
    }

    // print_disk(&disk);

    let mut back_position = disk.len();
    loop {
        if back_position == 0 {
            break;
        }
        back_position -= 1;
        let back_block = disk[back_position];
        match back_block {
            Block::File(_) => {
                let space_pos = next_space(&disk);
                if space_pos > back_position {
                    break;
                }
                disk[space_pos] = back_block;
                disk[back_position] = Block::Space;
                // print_disk(&disk);
            },
            Block::Space => {
                // noop
            },
        }        
    }
    // print_disk(&disk);

    checksum(&disk)
}

fn next_space(disk: &Vec<Block>) -> usize {
    let (left, _block) = disk.iter().find_position(|b| match b  {
        Block::File(_) => false,
        Block::Space => true,
    }).unwrap();
    left
}

#[allow(dead_code)]
fn print_disk(d: &Vec<Block>) {
    for b in d {
        match b {
            Block::File(id) => {
                print!("{id}");
            },
            Block::Space => {
                print!(".");
            },
        }
    }
    println!("");
}

fn checksum(disk: &Vec<Block>) -> usize {
    disk.iter().enumerate()
        .map(|(pos, block)| {
        // position multiplied by its file ID number
        match block {
            Block::File(id) => pos * id,
            Block::Space => 0,
        }
    }).sum()
}

fn part2(txt: &str) -> usize {
    let mut disk: Vec<Block> = Vec::new();
    let mut blocks = Vec::new();
    let mut pos = 0;

    for (i, c) in txt.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            let block = Block::File(i / 2);
            disk.extend(std::iter::repeat_n(block, size as usize));
            let contiguous_block = ContiguousBlock::File(File { 
                position: pos,
                size,
                id: i / 2 
            });
            blocks.push(contiguous_block);
        } else {
            let block = Block::Space;
            disk.extend(std::iter::repeat_n(block, size as usize));
            let contiguous_block = ContiguousBlock::Space(Space { 
                position: pos,
                size 
            });
            blocks.push(contiguous_block);
        };
        pos += size;
    }

    // print_disk(&disk);

    let mut back_position = blocks.len();

    loop {
        if back_position == 0 {
            break;
        }
        back_position -= 1;
        let b = (&blocks[back_position]).clone();
        // println!("back_position: {}, {:?}", back_position, b);
        match b {
            ContiguousBlock::File(File { position, size, id }) => {
                if let Some(found) = find_free_size(size, &mut blocks[0..back_position]) {
                    let block_idx = found.position;
                    // reduce the free blocks by the file size

                    found.position = found.position + size;
                    found.size = found.size - size;

                    for j in block_idx..block_idx + size {
                        disk[j] = Block::File(id);
                    }
                    for j in position..position + size {
                        disk[j] = Block::Space;
                    }
                    // print_disk(&disk);
                    // println!("blocks: {:?}", blocks[free_pos]);
                }                
            },
            ContiguousBlock::Space(_) => {
                // noop
            },
        }
    }

    // print_disk(&disk);

    checksum(&disk)
}

fn find_free_size(size: usize, blocks: &mut [ContiguousBlock]) -> Option<&mut Space> {
    for b in blocks {
        match b {
            ContiguousBlock::File(_file) => {
                // no match
            },
            ContiguousBlock::Space(space) => if space.size >= size {
                return Some(space);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    File(usize),
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContiguousBlock {
    File(File),
    Space(Space),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct File {
    position: usize,
    size: usize,
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Space {
    position: usize,
    size: usize
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(1928, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(6310675819476, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(2858, part2(test_input));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(6335972980679, part2(test_input));
    }
}
