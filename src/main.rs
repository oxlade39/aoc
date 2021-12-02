use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Position {
    vertical: i32,
    horizontal: i32,
    aim: i32
}

impl Default for Position {
    fn default() -> Self {
        Self { vertical: 0, horizontal: 0, aim: 0 }
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    BadText
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Position {
    fn up(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal, vertical: self.vertical, aim: self.aim - n}
    }
    fn down(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal, vertical: self.vertical, aim: self.aim + n}
    }
    fn forward(self, n: i32) -> Self {
        return Position{ horizontal: self.horizontal + n, vertical: self.vertical + self.aim * n, aim: self.aim}
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").into_iter().collect();
        if parts.len() != 2 {
            return Result::Err(ParseInstructionError::BadText);
        }
        let instruction = parts[0];
        let quantity: i32 = parts[1].parse().map_err(|_| ParseInstructionError::BadText)?;
        match instruction {            
            "forward" => Result::Ok(Instruction::Forward(quantity)),
            "up" => Result::Ok(Instruction::Up(quantity)),
            "down" => Result::Ok(Instruction::Down(quantity)),
            _ => Result::Err(ParseInstructionError::BadText)
        }        
    }
}

fn main() -> io::Result<()>{
    let lines = read_lines("./input/2.b.txt")?;
    let it = lines
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Instruction>().unwrap())
        .fold(Position::default(), |accum, instruction| {
            match instruction {
                Instruction::Up(n) => accum.up(n),
                Instruction::Down(n) => accum.down(n),
                Instruction::Forward(n) => accum.forward(n)
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