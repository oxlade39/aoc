use core::str;
use std::{
    fmt::{Debug, Write},
    i64,
    str::FromStr,
    time::Instant,
    usize,
};

use aoclib::timing;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> i64 {
    let machines: Vec<Machine> = txt.lines().map(|l| l.parse().unwrap()).collect();
    machines
        .iter()
        .fold(0, |accum, m| accum + m.min_light_presses())
}

fn part2(txt: &str) -> i64 {
    let machines: Vec<Machine> = txt.lines().map(|l| l.parse().unwrap()).collect();
    machines
        .iter()
        .fold(0, |accum, m| accum + m.min_jolt_presses())
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    lights: Indicator,
    wires: Vec<WireMask>,
    joltage: Joltage,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Indicator {
    mask: u16,
    len: usize,
}

impl Debug for Indicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len;
        let s = format!("[{:0width$b}]", self.mask, width = len);
        let as_indicator_string = s.replace("1", "#").replace("0", ".");
        f.write_str(&as_indicator_string)
    }
}

impl Debug for WireMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.1;
        let mut r = f.write_char('(');
        for i in 0..len {
            let mask = mask(i, len);
            if mask & self.0 == mask {
                if i == len - 1 {
                    r = r.and_then(|_| f.write_fmt(format_args!("{i}")));
                } else {
                    r = r.and_then(|_| f.write_fmt(format_args!("{i},")));
                }
            }
        }
        f.write_char(')')
    }
}

fn mask(i: usize, len: usize) -> u16 {
    1 << len - 1 - i
}

impl Indicator {
    fn apply(&self, wire: &WireMask) -> Indicator {
        let mask = self.mask ^ wire.0;

        Indicator {
            mask,
            len: self.len,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Wire(Vec<usize>);

#[derive(PartialEq, Eq, Clone, Copy)]
struct WireMask(u16, usize);

#[derive(Debug, PartialEq, Eq)]
struct Joltage(Vec<usize>);

#[derive(Debug, PartialEq, Eq)]
struct Machine2 {
    wires: Vec<Wire>,
    joltage: Joltage,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (front, back) = s.split_once(" ").unwrap();
        let (middle, back) = back.rsplit_once(" ").unwrap();
        let lights = front.parse::<Indicator>().expect("lights");
        let wires = middle
            .split(" ")
            .map(|chunk| chunk.parse::<Wire>().expect("wires"))
            .map(|w| w.to_mask(lights.len))
            .collect();
        Ok(Machine {
            lights,
            wires,
            joltage: back.parse().expect("joltage"),
        })
    }
}

impl FromStr for Indicator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..(s.len() - 1)];
        let mut b: u16 = 0;
        for i in 0..s.len() {
            let index = s.len() - i;
            let c = &s[index - 1..index];
            let bit = if c == "#" { 1 } else { 0 };
            b += bit << i
        }
        Ok(Indicator {
            mask: b,
            len: s.len(),
        })
    }
}

impl FromStr for Wire {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s[1..(s.len() - 1)]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Wire(nums))
    }
}

impl Wire {
    fn to_mask(self, len: usize) -> WireMask {
        let mut mask = 0;
        for w in self.0 {
            mask += 1 << (len - 1 - w);
        }
        WireMask(mask, len)
    }
}

impl FromStr for Joltage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s[1..(s.len() - 1)]
            .split(",")
            .map(|n| n.parse().expect(&format!("integers: {:?}", s)))
            .collect();
        Ok(Joltage(nums))
    }
}

impl FromStr for Machine2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, back) = s.split_once(" ").unwrap();
        let (middle, back) = back.rsplit_once(" ").unwrap();
        let wires: Vec<_> = middle
            .split(" ")
            .map(|chunk| chunk.parse::<Wire>().expect("wires"))
            .collect();
        let joltage = back.parse().expect("joltage");
        Ok(Machine2 { wires, joltage })
    }
}

impl Machine {
    fn min_light_presses(&self) -> i64 {
        let Self {
            lights,
            wires,
            joltage: _,
        } = self;
        let mut i = 0_i64;
        loop {
            i += 1;
            for combo in wires.iter().combinations_with_replacement(i as usize) {
                let all_off = Indicator {
                    mask: 0,
                    len: lights.len,
                };
                let end = combo.iter().fold(all_off, |accum, next| {
                    let n = accum.apply(next);
                    n
                });
                if end == *lights {
                    return i;
                }
            }
        }
    }

    fn min_jolt_presses(&self) -> i64 {
        let Self {
            lights,
            wires,
            joltage,
        } = self;
        let mut i = 0_i64;
        loop {
            i += 1;
            // if i > 10 {
            //     panic!("too high");
            // }
            // println!("trying buttons of size {} looking for {:?}", i, joltage.0);
            for combo in wires.iter().combinations_with_replacement(i as usize) {
                let mut counts = vec![0; lights.len];
                for wire in combo {
                    let mut state = Indicator {
                        mask: 0,
                        len: lights.len,
                    };
                    let after = state.apply(wire);
                    for i in 0..lights.len {
                        let mask = mask(i, lights.len);
                        if (mask & after.mask) == mask {
                            counts[i] += 1;
                        }
                    }
                    // println!(
                    //     "{:?} -> {:?} | applying {:?} looking for {:?}",
                    //     state, after, wire, joltage.0
                    // );
                    state = after;
                }
                // println!("[{}] result was {:?}", i, counts);
                if counts == joltage.0 {
                    return i;
                }
            }
        }
    }
}

impl Machine2 {
    fn count(&self) -> i64 {
        let max = *self.wires.iter().flat_map(|w| w.0.iter()).max().unwrap();
        let buttons_by_wire_idx: Vec<Vec<Wire>> = (0..max)
            .map(|i| {
                let v: Vec<_> = self
                    .wires
                    .iter()
                    .filter(|w| w.0.contains(&i))
                    .cloned()
                    .collect();
                v
            })
            .collect();

        todo!();
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_all() {
        let test_input = include_str!("input.test.txt");
        let ms: Vec<_> = test_input
            .lines()
            .map(|l| l.parse::<Machine>().unwrap())
            .collect();
        for m in ms {
            println!("{:?}", m);
        }
    }

    #[test]
    fn test_parse_indicator() {
        let txt = "[.##.]";
        let i: Indicator = txt.parse().unwrap();
        let as_bits = format!("{:b}", i.mask);
        assert_eq!("110", as_bits);
        assert_eq!(Indicator { mask: 6, len: 4 }, i);

        let txt = "[...#.]";
        let i: Indicator = txt.parse().unwrap();
        let as_bits = format!("{:b}", i.mask);
        assert_eq!("10", as_bits);
        assert_eq!(5, i.len);

        let txt = "[.###.#]";
        let i: Indicator = txt.parse().unwrap();
        let as_bits = format!("{:b}", i.mask);
        assert_eq!("11101", as_bits);
        assert_eq!(6, i.len);
    }

    #[test]
    fn test_parse() {
        let s = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let m = s.parse();
        let expected = Ok(Machine {
            lights: Indicator { mask: 6, len: 4 },
            wires: vec![
                WireMask(1, 4),
                WireMask(5, 4),
                WireMask(2, 4),
                WireMask(3, 4),
                WireMask(10, 4),
                WireMask(12, 4),
            ],
            joltage: Joltage(vec![3, 5, 4, 7]),
        });
        assert_eq!(expected, m);

        assert_eq!(2, expected.unwrap().min_light_presses());
    }

    #[test]
    fn test_wire_mask() {
        let w = Wire(vec![3]);
        let m = w.to_mask(4);
        assert_eq!("1", format!("{:b}", m.0));

        let w = Wire(vec![1, 3]);
        let m = w.to_mask(4);
        assert_eq!("101", format!("{:b}", m.0));

        let w = Wire(vec![0, 1]);
        let m = w.to_mask(4);
        assert_eq!("1100", format!("{:b}", m.0));
        assert_eq!(12, m.0);
    }

    #[test]
    fn test_apply_mask() {
        let start: Indicator = "[....]".parse().unwrap();
        let i: Indicator = "[.##.]".parse().unwrap();
        let w = Wire(vec![3]).to_mask(4);

        let next = start.apply(&w);
        let expected: Indicator = "[...#]".parse().unwrap();
        assert_eq!(expected, next);

        let next = next.apply(&w);
        let expected: Indicator = "[....]".parse().unwrap();
        assert_eq!(expected, next);

        let first_three = vec![
            Wire(vec![3]).to_mask(4),
            Wire(vec![1, 3]).to_mask(4),
            Wire(vec![2]).to_mask(4),
        ];
        let end = first_three
            .iter()
            .fold(start, |accum, next| accum.apply(next));
        assert_eq!(i, end);
    }

    #[test]
    fn test_debug_wiremask() {
        let m = Wire(vec![0, 2, 3, 4]);
        let wm = m.to_mask(5);
        assert_eq!("(0,2,3,4)", format!("{:?}", wm));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(7, part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(517, part1(test_input));
    }

    #[test]
    fn test_input_pt2() {
        let test_input = include_str!("input.test.txt");
        assert_eq!(33, part2(test_input));
    }

    #[test]
    fn input_pt2_examples() {
        let m: Machine = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
            .parse()
            .unwrap();
        let i = m.min_jolt_presses();
        assert_eq!(10, i);
    }

    // too slow
    // #[test]
    // fn input_pt2() {
    //     let test_input = include_str!("input.txt");
    //     assert_eq!(0, part2(test_input));
    // }
}
