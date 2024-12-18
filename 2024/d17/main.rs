use core::str;
use std::{i64, io, str::FromStr, time::Instant, usize};

use aoclib::{input, timing};
use itertools::Itertools;

use std::io::{BufRead};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> String {
    let mut lines_itr = txt.lines();
    let (_, a) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let (_, b) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let (_, c) = lines_itr.next().unwrap().split_once(": ").unwrap();
    lines_itr.next().unwrap();

    let (_, program_txt) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let program: Vec<_> = program_txt.split(",").map(|n| n.parse().unwrap()).collect();

    let mut reg = Registers { 
        a: a.parse().unwrap(), 
        b: b.parse().unwrap(), 
        c: c.parse().unwrap(), 
        output: String::new() 
    };

    reg.process(0, &program);

    reg.output
}

fn part2(txt: &str) -> i64 {
    let mut lines_itr = txt.lines();
    let (_, a) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let (_, b) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let (_, c) = lines_itr.next().unwrap().split_once(": ").unwrap();
    lines_itr.next().unwrap();

    let (_, program_txt) = lines_itr.next().unwrap().split_once(": ").unwrap();
    let program: Vec<_> = program_txt.split(",").map(|n| n.parse().unwrap()).collect();

    let mut reg = Registers { 
        a: a.parse().unwrap(), 
        b: b.parse().unwrap(), 
        c: c.parse().unwrap(), 
        output: String::new() 
    };

    let mut reg_a_value = 0;

    // 164278585000000 => 4,0,4,4,4,0,3,2,0,2,5,5,0,3,3,0
    // ANSWER
    // 164278496489149 => 2,4,1,1,7,5,1,5,4,2,5,5,0,3,3,0

    let mut i = 164278585000000;

    loop {
        let mut reg = Registers {
            a: i,
            b: 0,
            c: 0,
            output: String::new(),
        };
        reg.process(0, &program);

        if reg.output == "2,4,1,1,7,5,1,5,4,2,5,5,0,3,3,0" {
            println!("{} => {}", i, reg.output);
            // break;            
        }        
        i -= 1;

        // if i % 1000000 == 0 {
        //     println!("{} => {}", i, reg.output);
        // }
    }
    
    // let mut i = 0;

    // println!("enter an incrememnt or decrement");
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let line_str = line.unwrap();
    //     let plus_minus = &line_str[0..1];
    //     match plus_minus {
    //         "+" => {
    //             let increment: i64 = line_str[1..].parse().unwrap();
    //             i = i + increment;
    //             reg.a = i;
    //             reg.b = 0;
    //             reg.c = 0;
    //             reg.output = String::new();
    //             reg.process(0, &program);
    //             println!("{} => {}", i, reg.output);
    //         },
    //         "-" => {
    //             let decrement: i64 = line_str[1..].parse().unwrap();
    //             i = i - decrement;
    //             reg.a = i;
    //             reg.b = 0;
    //             reg.c = 0;
    //             reg.output = String::new();
    //             reg.process(0, &program);
    //             println!("{} => {}", i, reg.output);
    //         },
    //         _ignored => {
    //             println!("ignored");
    //         },
    //     }
    // }

    reg_a_value
}

enum ComboOperand {
    Literal(i64),
    RegisterA,
    RegisterB,
    RegisterC,
    Reserved,
}

impl From<usize> for ComboOperand {
    fn from(value: usize) -> Self {
        match value {
            0 => ComboOperand::Literal(0),
            1 => ComboOperand::Literal(1),
            2 => ComboOperand::Literal(2),
            3 => ComboOperand::Literal(3),
            4 => ComboOperand::RegisterA,
            5 => ComboOperand::RegisterB,
            6 => ComboOperand::RegisterC,
            7 => ComboOperand::Reserved,
            other => panic!("bad operand '{}'", other),
        }
    }
}

enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<usize> for Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            other => panic!("bad instruction code: {other}")
        }
    }
}

struct Registers {
    a: i64,
    b: i64,
    c: i64,
    output: String,
}

impl Registers {
    fn process(
        &mut self,
        instuction_pointer: usize,
        program: &Vec<usize>
    ) {
        if instuction_pointer >= program.len(){
            return;
        }

        let instruction: Instruction = program[instuction_pointer].into();
        match instruction {
            Instruction::ADV => {
                // The adv instruction (opcode 0) performs division. 
                // The numerator is the value in the A register. 
                // The denominator is found by raising 2 to the power of the instruction's combo operand. 
                // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) 
                // The result of the division operation is truncated to an integer and then written to the A register.
                let operand: ComboOperand = program[instuction_pointer + 1].into();
                let numerator = self.a;
                let operand_value = match operand {
                    ComboOperand::Literal(i) => i,
                    ComboOperand::RegisterA => self.a,
                    ComboOperand::RegisterB => self.b,
                    ComboOperand::RegisterC => self.c,
                    ComboOperand::Reserved => panic!("reserved used"),
                };
                let result = numerator / 2_i64.pow(operand_value as u32);
                self.a = result;
                self.process(instuction_pointer + 2, program);
            },
            Instruction::BXL => {
                // The bxl instruction (opcode 1) calculates the bitwise XOR of register B 
                // and the instruction's literal operand, then stores the result in register B.
                let register_b = self.b;
                let operand = program[instuction_pointer + 1] as i64;
                let result = register_b ^ operand;
                self.b = result;
                self.process(instuction_pointer + 2, program);
            },
            Instruction::BST => {
                // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 
                // (thereby keeping only its lowest 3 bits), 
                // then writes that value to the B register.
                let operand: ComboOperand = program[instuction_pointer + 1].into();
                let operand_value = match operand {
                    ComboOperand::Literal(i) => i,
                    ComboOperand::RegisterA => self.a,
                    ComboOperand::RegisterB => self.b,
                    ComboOperand::RegisterC => self.c,
                    ComboOperand::Reserved => panic!("reserved used"),
                };
                let result = operand_value % 8;
                self.b = result;
                self.process(instuction_pointer + 2, program);
            },
            Instruction::JNZ => {
                // The jnz instruction (opcode 3) does nothing if the A register is 0. 
                // However, if the A register is not zero, it jumps by setting the instruction pointer 
                // to the value of its literal operand; 
                // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                if self.a != 0 {
                    let literal_operand = program[instuction_pointer + 1];
                    self.process(literal_operand, program);
                } else {
                    self.process(instuction_pointer + 2, program);
                }
            },
            Instruction::BXC => {
                // The bxc instruction (opcode 4) calculates 
                // the bitwise XOR of register B and register C, 
                // then stores the result in register B. 
                // (For legacy reasons, this instruction reads an operand but ignores it.)
                let register_b = self.b;
                let register_c = self.c;
                let result = register_b ^ register_c;
                self.b = result;
                self.process(instuction_pointer + 2, program);
            },
            Instruction::OUT => {
                // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, 
                // then outputs that value. 
                // (If a program outputs multiple values, they are separated by commas.)
                let operand: ComboOperand = program[instuction_pointer + 1].into();
                let operand_value = match operand {
                    ComboOperand::Literal(i) => i,
                    ComboOperand::RegisterA => self.a,
                    ComboOperand::RegisterB => self.b,
                    ComboOperand::RegisterC => self.c,
                    ComboOperand::Reserved => panic!("reserved used"),
                };
                let result = operand_value % 8;
                if self.output.len() > 0 {
                    self.output.push(',');
                }
                self.output.push_str(format!("{}", result).as_str());
                self.process(instuction_pointer + 2, program);
            },
            Instruction::BDV => {
                // The bdv instruction (opcode 6) 
                // works exactly like the adv instruction 
                // except that the result is stored in the B register. 
                // (The numerator is still read from the A register.)
                let operand: ComboOperand = program[instuction_pointer + 1].into();
                let numerator = self.a;
                let operand_value = match operand {
                    ComboOperand::Literal(i) => i,
                    ComboOperand::RegisterA => self.a,
                    ComboOperand::RegisterB => self.b,
                    ComboOperand::RegisterC => self.c,
                    ComboOperand::Reserved => panic!("reserved used"),
                };
                let result = numerator / 2_i64.pow(operand_value as u32);
                self.b = result;
                self.process(instuction_pointer + 2, program);
            },
            Instruction::CDV => {
                // The cdv instruction (opcode 7) 
                // works exactly like the adv instruction 
                // except that the result is stored in the C register. 
                // (The numerator is still read from the A register.)
                let operand: ComboOperand = program[instuction_pointer + 1].into();
                let numerator = self.a;
                let operand_value = match operand {
                    ComboOperand::Literal(i) => i,
                    ComboOperand::RegisterA => self.a,
                    ComboOperand::RegisterB => self.b,
                    ComboOperand::RegisterC => self.c,
                    ComboOperand::Reserved => panic!("reserved used"),
                };
                let result = numerator / 2_i64.pow(operand_value as u32);
                self.c = result;
                self.process(instuction_pointer + 2, program);
            },
        }
    }
}

#[cfg(test)]
mod tests {    
    use std::process::Output;

    use crate::*;

    #[test]
    fn test_smaller_examples() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut reg = Registers { a: 1, b: 1, c: 9, output: String::new() };
        reg.process(0, &vec![2,6]);
        assert_eq!(1, reg.b);
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut reg = Registers { a: 10, b: 1, c: 1, output: String::new() };
        reg.process(0, &vec![5,0,5,1,5,4]);
        assert_eq!("0,1,2", reg.output);
        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let mut reg = Registers { a: 2024, b: 1, c: 1, output: String::new() };
        reg.process(0, &vec![0,1,5,4,3,0]);
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", reg.output);
        assert_eq!(0, reg.a);
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(test_input));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!("3,6,7,0,5,7,3,1,4", part1(test_input));
    }

    // #[test]
    // fn test_input_pt2() {
    //     let test_input = include_str!("input.repeat.txt");
    //     assert_eq!(117440, part2(test_input));
    // }

    // #[test]
    // fn input_pt2() {
    //     let test_input = include_str!("input.txt");
    //     assert_eq!(0, part2(test_input));
    // }
}
