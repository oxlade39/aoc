use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
    time::Instant,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> u64 {
    txt.split(",")
        .map(|chunk| chunk.parse::<Operation>().unwrap())
        .map(|op| op.hash() as u64)
        .sum::<u64>()
}

fn part2(txt: &str) -> i64 {
    let mut boxes = Boxes::new();

    for operation in txt.split(",").map(|op| op.parse::<Operation>().unwrap()) {
        boxes.apply(operation);
    }

    boxes.focussing_power()
}

#[derive(Debug)]
enum Operation {
    Remove(String),
    Set(String, i8),
}

impl Operation {
    fn label(&self) -> &str {
        match self {
            Operation::Remove(label) => &label[0..],
            Operation::Set(label, _) => &label[0..],
        }
    }
}

#[derive(Debug, Clone)]
struct Box {
    labels: Vec<String>,
    lenses: HashMap<String, i8>,
}

#[derive(Debug)]
struct Boxes(Vec<Box>);

impl Boxes {
    fn new() -> Boxes {
        Boxes(vec![
            Box {
                labels: vec![],
                lenses: HashMap::new()
            };
            256
        ])
    }

    fn apply(&mut self, operation: Operation) {
        let box_num = operation.label().hash();
        let b = &mut self.0[box_num as usize];

        match operation {
            Operation::Remove(label) => {
                if let Some(_existing) = b.lenses.remove(&label) {
                    b.labels = b
                        .labels
                        .clone()
                        .into_iter()
                        .filter(|l| l != &label)
                        .collect();
                }
            }
            Operation::Set(label, value) => {
                if let Some(existing) = b.lenses.get_mut(&label) {
                    *existing = value;
                } else {
                    b.lenses.insert(label.clone(), value);
                    b.labels.push(label)
                }
            }
        }
    }

    fn focussing_power(&self) -> i64 {
        let mut total = 0;
        for (i, b) in self.0.iter().enumerate() {
            let lens_n = (i + 1) as i64;
            for (j, slot) in b.labels.iter().enumerate() {
                let slot_num = (j + 1) as i64;
                let focal_length = b
                    .lenses
                    .get(slot)
                    .expect(&format!("expect slot for label {} in box {}", slot, i));

                let my_focussing_power = lens_n * slot_num * (*focal_length as i64);
                total += my_focussing_power;
            }
        }

        total
    }
}

trait Hash {
    fn hash(&self) -> u8;
}

impl<T> Hash for T
where
    T: Display,
{
    fn hash(&self) -> u8 {
        let str = &format!("{}", self)[0..];
        let mut chunk_sum = 0;
        for c in str.chars() {
            let ascii = c as i64;
            chunk_sum += ascii;
            chunk_sum *= 17;
            chunk_sum %= 256;
        }
        chunk_sum as u8
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Remove(label) => f.write_fmt(format_args!("{}-", label)),
            Operation::Set(label, focal_length) => {
                f.write_fmt(format_args!("{}={}", label, focal_length))
            }
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let add = s.split("=").collect_vec();
        if add.len() == 2 {
            let label = add[0].to_owned();
            let focal_length = add[1].parse::<i8>().unwrap();
            return Ok(Operation::Set(label, focal_length));
        }

        let remove = s.split("-").collect_vec();
        if remove.len() == 2 {
            let label = remove[0].to_owned();
            return Ok(Operation::Remove(label));
        }

        Err(format!("bad input: {s}").to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(1320, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(145, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_hash() {
        let op: Operation = "rn=1".parse().unwrap();
        assert_eq!(30, op.hash());
        let op: Operation = "cm-".parse().unwrap();
        assert_eq!(253, op.hash());
    }

    #[test]
    fn test_apply_to_boxes() {
        let mut boxes = Boxes::new();
        let op = "rn=1".parse::<Operation>().unwrap();
        boxes.apply(op);

        assert_eq!("rn", boxes.0[0].labels[0]);

        let op = "cm-".parse::<Operation>().unwrap();
        boxes.apply(op);

        assert_eq!("rn", boxes.0[0].labels[0]);

        let op = "qp=3".parse::<Operation>().unwrap();
        boxes.apply(op);

        assert_eq!("rn", boxes.0[0].labels[0]);
        assert_eq!("qp", boxes.0[1].labels[0]);

        let op = "cm=2".parse::<Operation>().unwrap();
        boxes.apply(op);

        assert_eq!("rn", boxes.0[0].labels[0]);
        assert_eq!("cm", boxes.0[0].labels[1]);
        assert_eq!("qp", boxes.0[1].labels[0]);

        let op = "qp-".parse::<Operation>().unwrap();
        boxes.apply(op);

        assert_eq!("rn", boxes.0[0].labels[0]);
        assert_eq!("cm", boxes.0[0].labels[1]);
        assert_eq!(0, boxes.0[1].labels.len());
    }
}
