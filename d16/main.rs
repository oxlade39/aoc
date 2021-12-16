use std::{num::ParseIntError, fmt::Write, time::Instant};


fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {}ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");
    let input_as_bin = hex_to_bin(input);
    let result = parse(&input_as_bin);
    println!("part1: {:?}", sum(&result));
}

fn part2() {
}

#[test]
fn test_hex() {
    let s = "D2FE28";
    let result = hex_to_bin(s);
    assert_eq!("110100101111111000101000", result);
    assert_eq!(hex_to_bin("38006F45291200"), "00111000000000000110111101000101001010010001001000000000")
}

#[test]
fn test_bin_to_int() {
    assert_eq!(bin_to_i16("110"), 6);
    assert_eq!(bin_to_i16("100"), 4);
}

#[derive(Debug)]
struct Version(i16);

#[derive(Debug)]
struct TypeId(i16);

#[derive(Debug)]
enum Instruction {
    Operator(i16, Vec<Instruction>),
    Literal(i16, Vec<String>)
}

fn parse_literal(s: &str, version: i16, to_append: &mut Vec<Instruction>) -> usize {
    let mut position = 0;
    let mut parts: Vec<String> = Vec::new();

    loop {
        let first = s.chars().nth(position).unwrap();
        if first == '1' {
            parts.push(s[position+1..position+5].to_string());
            position += 5;
        } else {
            parts.push(s[position+1..position+5].to_string());
            position += 5;

            break;
        }
    }

    let lit = Instruction::Literal(version, parts);
    println!("appending literal: {:?}", lit);
    to_append.push(lit);
    
    position
}

fn parse(s: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    parse_internal(s, None, &mut instructions);
    instructions
}


fn parse_internal(s: &str, max: Option<usize>, instructions: &mut Vec<Instruction>) -> usize {
    println!("parse: {}", s);
    let mut position = 0;

    let mut remaining = max.clone();

    loop {
        if position + 6 > s.len() {
            break;
        }

        if let Some(remaining) = remaining {
            if remaining == 0 {
                return position;
            }
        }
        remaining = remaining.map(|i| i-1);

        let v = bin_to_i16(&s[position..position+3]);
        let t = bin_to_i16(&s[position+3..position+6]);

        println!("[{}] v:{} t:{}", position, v, t);

        position += 6;
        if t == 4 {        
            let inc = parse_literal(&s[position..], v, instructions);
            position += inc;
        } else {
            if let Some(i) = s.chars().nth(position) {
                position += 1;
                if i == '0' {
                    // 15 bits, n-bytes
                    if position + 15 > s.len() {
                        continue;
                    }
                    let length = bin_to_usize(&s[position..position+15]);
                    position += 15;
                    if position + length > s.len() {
                        continue;
                    }                    
                    let children = parse(&s[position..position+length]);
                    instructions.push(Instruction::Operator(v, children));
                    position += length;
                } else {
                    // 11 bits, n-packets
                    if position + 11 > s.len() {
                        continue;
                    }
                    let length = bin_to_usize(&s[position..position+11]);
                    position += 11;

                    println!("looking for {} subpackets in {}", length, &s[position..]);
                    
                    let mut children = Vec::new();
                    let inc = parse_internal(&s[position..], Some(length), &mut children);
                    instructions.push(Instruction::Operator(v, children));
                    position += inc;
                }
            }            
        }
    }
    position
}

fn sum(i: &Vec<Instruction>) -> i64 {
    let mut total: i64 = 0;

    for instruction in i {
        match instruction {
            Instruction::Literal(version, _) => total += *version as i64,
            Instruction::Operator(version, children) => {
                total += sum(children) + *version as i64;
            }
        }
    }
    
    total
}

#[test]
fn test_parse_literal() {
    println!("{:?}", parse("110100101111111000101000"));
    println!("{:?}", parse("11010001010"));
    println!("{:?}", parse("0101001000100100"));
}

#[test]
fn test_parse_operator_15() {
    let result = parse("00111000000000000110111101000101001010010001001000000000");
    println!("{:?}", result);
    assert_eq!(result.len(), 1);
}


#[test]
fn test_parse_operator_11() {
    let result = parse("11101110000000001101010000001100100000100011000001100000");
    println!("{:?}", result);
}

#[test]
fn test_pt1_examples() {
    let mut result = parse(&hex_to_bin("8A004A801A8002F478"));
    println!("****{:?} -> {}", &result, sum(&result));
    assert_eq!(16, sum(&result));

    result = parse(&hex_to_bin("620080001611562C8802118E34"));
    println!("****{:?} -> {}", &result, sum(&result));
    assert_eq!(12, sum(&result));

    result = parse(&hex_to_bin("C0015000016115A2E0802F182340"));
    println!("****{:?} -> {}", &result, sum(&result));
    assert_eq!(23, sum(&result));

    result = parse(&hex_to_bin("A0016C880162017C3686B18A3D4780"));
    println!("****{:?} -> {}", &result, sum(&result));
    assert_eq!(31, sum(&result));

}

fn hex_to_bin(s: &str) -> String {
    (0..s.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .map(|i| format!("{:0>8b}", i))
        .collect()
}

fn bin_to_i16(s: &str) -> i16 {
    i16::from_str_radix(s, 2).unwrap()
}

fn bin_to_usize(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}