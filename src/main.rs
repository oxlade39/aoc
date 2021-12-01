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

fn main() -> io::Result<()>{
    let lines = read_lines("./input/1.b.test.txt")?;
    let it = lines
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Depth>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .fold(Measurement::default(), |accum, depths| {
            let sum = depths.iter().sum();
            match accum.prev_depth {
                None => Measurement{ prev_depth: Some(sum), count: 0},
                Some(previous) => if sum > previous {
                    Measurement{ prev_depth: Some(sum), count: accum.count + 1 }
                } else {
                    Measurement{ prev_depth: Some(sum), count: accum.count }
                }
            }
        });

    println!("result: {:?}", it);
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}