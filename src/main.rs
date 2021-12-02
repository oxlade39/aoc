use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Sum;
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd)]
struct Depth(i16);

impl From<i16> for Depth {
    fn from(d: i16) -> Self {
        Depth(d)
    }
}

#[derive(Debug)]
struct Measurement {
    prev_depth: Option<Depth>,
    count: i16
}

impl Default for Measurement {
    fn default() -> Self {
        Self { prev_depth: None, count: 0 }
    }
}

#[derive(Debug)]
struct Position {
    vertical: i32,
    horizontal: i32
}

impl Default for Position {
    fn default() -> Self {
        Self { vertical: 0, horizontal: 0 }
    }
}

impl FromStr for Depth {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i: i16 = s.parse()?;
        Ok(i.into())
    }
}

impl<'a> Sum<&'a Depth> for Depth {
    fn sum<I: Iterator<Item = &'a Depth>>(iter: I) -> Self {
        iter.map(|d|d.0).sum::<i16>().into()
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    BADTEXT
}

#[derive(Debug)]
enum Instruction {
    FORWARD(i32),
    UP(i32),
    DOWN(i32),
}

impl Position {
    fn up(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal, vertical: self.vertical - n}
    }
    fn down(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal, vertical: self.vertical + n}
    }
    fn forward(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal + n, vertical: self.vertical}
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").into_iter().collect();
        if parts.len() != 2 {
            return Result::Err(ParseInstructionError::BADTEXT);
        }
        let instruction = parts[0];
        let quantity: i32 = parts[1].parse().map_err(|_| ParseInstructionError::BADTEXT)?;
        match instruction {            
            "forward" => Result::Ok(Instruction::FORWARD(quantity)),
            "up" => Result::Ok(Instruction::UP(quantity)),
            "down" => Result::Ok(Instruction::DOWN(quantity)),
            _ => Result::Err(ParseInstructionError::BADTEXT)
        }        
    }
}

fn main() -> io::Result<()>{
    let lines = read_lines("./input/2.a.txt")?;
    let it = lines
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Instruction>().unwrap())
        .fold(Position::default(), |accum, instruction| {
            println!("{:?} -> {:?}", accum, instruction);
            match instruction {
                Instruction::UP(n) => accum.up(n),
                Instruction::DOWN(n) => accum.down(n),
                Instruction::FORWARD(n) => accum.forward(n)
            }
        });

    println!("result: {:?}", it.vertical * it.horizontal);
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}