use std::{str::FromStr, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let part1_result = part1(input);
    println!("part1: {}", part1_result);
    let part2_result = part2(input);
    println!("part2: {}", part2_result);
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

#[derive(Debug, PartialEq, Clone)]
struct Operation {
    lhs: Value,
    rhs: Value,
    operand: Operand
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Old,
    Number(i64),
}

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Multiply,
    Subtract,
    Add,
    Divide
}

#[derive(Debug, PartialEq, Clone)]
struct Test {
    test_div_by: i64,
    true_throw: usize,
    false_throw: usize
}

#[derive(Debug, PartialEq, Clone)]
enum MonkeyParseError {
    BadLineCount(usize),
    BadId,
    BadWorryLevel(String),
    BadOperation,
    BadTest(String),
}

impl FromStr for Monkey {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = s.lines().collect();
        if sections.len() < 6 {
            Err(MonkeyParseError::BadLineCount(sections.len()))
        } else {
            let id: usize = if let Some(n) = sections[0].split(":").next() {
                n.parse().map_err(|_| MonkeyParseError::BadId)?
            } else {
                Err(MonkeyParseError::BadId)?
            };

            let starting_items: Vec<i64> = sections[1]
                .trim()
                .split(" ")
                .skip(2)
                .map(|worry_level| worry_level.replace(",", "").parse().map_err(|_| MonkeyParseError::BadWorryLevel(format!("non int: {}", worry_level))))
                .collect::<Result<Vec<i64>, self::MonkeyParseError>>()?;

            let operation: Operation = sections[2]
                .trim()
                .parse()
                .map_err(|_| MonkeyParseError::BadOperation)?;

            let rest = sections[3..].join("\n");
            let test: Test = rest.parse().map_err(|e| MonkeyParseError::BadTest(e))?;
            
            Ok(Monkey {
                id,
                items: starting_items,
                operation,
                test
            })
        }
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Value::Old),
            other => {
                let i: i64 = other.parse().map_err(|_| format!("bad operation value: {}", other))?;
                Ok(Value::Number(i))
            }
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ").skip(3);        
        let lhs: Value = parts.next().map_or(Err(format!("missing lhs")), |part| part.parse())?;
        let operand = match parts.next() {
            Some("*") => Ok(Operand::Multiply),
            Some("/") => Ok(Operand::Divide),
            Some("-") => Ok(Operand::Subtract),
            Some("+") => Ok(Operand::Add),
            other => Err(format!("bad operand {:?}", other)),
        }?;
        let rhs: Value = parts.next().map_or(Err(format!("missing rhs")), |part| part.parse())?;
        Ok(Operation { 
            lhs,
            rhs,
            operand,
        })
    }
}

impl FromStr for Test {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_itr = s.lines();
        let test_div_by = match lines_itr.next() {
            Some(line) => {
                let result = line
                    .trim()
                    .split(" ")
                    .skip(3)
                    .next()
                    .map_or(Err(format!("bad test_div_by in {}", s)), 
                        |n| n.parse().or(Err(format!("bad test_div_by in {}", s))));
                Ok(result?)
            }
            None => Err(format!("no lines in {:?}", s))
        }?;
        let true_throw: usize = match lines_itr.next() {
            Some(line) => {
                let result = line.trim().split(" ")
                    .skip(5)
                    .next()                    
                    .map_or(Err(format!("bad true throw in {}", s)),
                        |n| n.parse().or(Err(format!("bad true throw in {}", s))));
                Ok(result?)
            },
            None => Err(format!("no lines in {:?}", s))
        }?;
        let false_throw: usize = match lines_itr.next() {
            Some(line) => {
                let result = line.trim().split(" ")
                    .skip(5)
                    .next()                    
                    .map_or(Err(format!("bad false throw in {}", s)), 
                        |n| n.parse().or(Err(format!("bad false throw in {}", s))));
                Ok(result?)
            },
            None => Err(format!("no lines in {:?}", s))
        }?;
        Ok(Test { test_div_by, true_throw, false_throw })
    }
}

impl Operand {
    fn perform(&self, lhs: i64, rhs: i64) -> i64 {
        let result = match self {
            Operand::Add => lhs + rhs,
            Operand::Subtract => lhs - rhs,
            Operand::Multiply => lhs * rhs,
            Operand::Divide => lhs / rhs,
        };        
        result
    }
}

impl Value {
    fn materialise(&self, old: i64) -> i64 {
        match self {
            Value::Old => old,
            Value::Number(x) => *x
        }
    }
}

impl Operation {
    fn materialise(&self, old: i64) -> i64 {
        let lhs = self.lhs.materialise(old);
        let rhs = self.rhs.materialise(old);
        self.operand.perform(lhs, rhs)
    }
}

impl Test {
    fn apply(&self, worry_level: i64) -> usize {
        let result = worry_level % self.test_div_by;
        if result == 0 {
            self.true_throw
        } else {
            self.false_throw
        }
    }
}

impl Monkey {
    fn process_items<T>(
        &mut self, 
        worry_reduction: &T
    ) -> HashMap<usize, Vec<i64>> 
    where T: WorryReduction
    {
        let mut outbound: HashMap<usize, Vec<i64>> = HashMap::new();
        for item in &self.items {
            let next_worry_level = worry_reduction.reduce_worry(self.operation.materialise(*item));
            let next_monkey = self.test.apply(next_worry_level);
            if let Some(x) = outbound.get_mut(&next_monkey) {
                x.push(next_worry_level);
            } else {
                outbound.insert(next_monkey, vec![next_worry_level]);
            }
        }
        self.items = vec![];
        outbound
    }
}

trait WorryReduction {
    fn reduce_worry(&self, worry: i64) -> i64;
}

struct DivideWorry(i64);

impl WorryReduction for DivideWorry {
    fn reduce_worry(&self, worry: i64) -> i64 {
        worry / self.0
    }
}

struct ModWorry(i64);

impl WorryReduction for ModWorry {
    fn reduce_worry(&self, worry: i64) -> i64 {
        worry % self.0
    }
}


fn play<T>(
    monkeys: &mut Vec<Monkey>, 
    counts: &mut Vec<usize>, 
    wrd: &T) 
    where T: WorryReduction
    {
    for i in 0..monkeys.len() {
        counts[i] += monkeys[i].items.len();
        let thrown = monkeys[i].process_items(wrd);
        for (k,v) in thrown {
            for item in v {
                monkeys[k].items.push(item);
            }
        }
    }
}

fn part1(
    input: &str
) -> usize 
{
    let rounds = 20;
    let mut monkeys: Vec<Monkey> = input
            .split("Monkey ")
            .skip(1)
            .map(|monkey| monkey.parse().unwrap())
            .collect();

    let mut counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        play(&mut monkeys, &mut counts, &DivideWorry(3));
    }
    
    counts.sort();    
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

fn part2(
    input: &str
) -> usize 
{
    let rounds = 10000;
    let mut monkeys: Vec<Monkey> = input
            .split("Monkey ")
            .skip(1)
            .map(|monkey| monkey.parse().unwrap())
            .collect();

    let worry_reduction = monkeys
        .iter()
        .fold(1, |accum, monkey| {
            accum * monkey.test.test_div_by
        });

    let mut counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        play(&mut monkeys, &mut counts, &ModWorry(worry_reduction));
    }
    
    counts.sort();    
    counts[counts.len() - 1] * counts[counts.len() - 2]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_operation() {
        let result = "Operation: new = old * 19".parse();
        assert_eq!(result, 
            Ok(Operation{ 
                lhs: Value::Old, 
                operand: Operand::Multiply, 
                rhs: Value::Number(19)
            })
        );

        let result = "Operation: new = old / 1".parse();
        assert_eq!(result, 
            Ok(Operation{ 
                lhs: Value::Old, 
                operand: Operand::Divide, 
                rhs: Value::Number(1)
            })
        );

        let result = "Operation: new = 10 + 1".parse();
        assert_eq!(result, 
            Ok(Operation{ 
                lhs: Value::Number(10), 
                operand: Operand::Add, 
                rhs: Value::Number(1)
            })
        );

        let result = "Operation: new = 10 - old".parse();
        assert_eq!(result, 
            Ok(Operation{ 
                lhs: Value::Number(10), 
                operand: Operand::Subtract, 
                rhs: Value::Old
            })
        )
    }

    #[test]
    fn parse_test() {
        let result = 
        "Test: divisible by 17\n\
          If true: throw to monkey 0\n\
          If false: throw to monkey 1".parse();
        assert_eq!(result,
            Ok(Test{ 
                test_div_by: 17,
                true_throw: 0,
                false_throw: 1
            })
        )
    }

    #[test]
    fn parse_monkey() {
        let input = "0:\n\
        Starting items: 79, 98\n\
        Operation: new = old * 19\n\
        Test: divisible by 23\n\
          If true: throw to monkey 2\n\
          If false: throw to monkey 3";
        assert_eq!(input.parse(), Ok(Monkey { 
            id: 0,
            items: vec![79, 98],
            operation: Operation { lhs: Value::Old, rhs: Value::Number(19), operand: Operand::Multiply },
            test: Test { test_div_by: 23, true_throw: 2, false_throw: 3 }
        }))
    }

    #[test]
    fn parse() {
        let input = include_str!("input.example.txt");
        let monkeys: Vec<Monkey> = input
            .split("Monkey ")
            .skip(1)
            .map(|monkey| monkey.parse().unwrap())
            .collect();
        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[3], Monkey {
            id: 3,
            items: vec![74],
            operation: Operation { lhs: Value::Old, rhs: Value::Number(3), operand: Operand::Add },
            test: Test { test_div_by: 17, true_throw: 0, false_throw: 1 }
        });
    }

    #[test]
    fn perform_operands() {
        assert_eq!(Operand::Add.perform(1, 2), 3);
        assert_eq!(Operand::Subtract.perform(2, 1), 1);
        assert_eq!(Operand::Multiply.perform(10, 2), 20);
        assert_eq!(Operand::Divide.perform(10, 3), 3);
    }

    #[test]
    fn perform_operands_divide_rounds_down() {        
        assert_eq!(Operand::Divide.perform(1, 1), 1);
        assert_eq!(Operand::Divide.perform(1, 2), 0);
        assert_eq!(Operand::Divide.perform(1, 3), 0);
        assert_eq!(Operand::Divide.perform(3, 2), 1);
        assert_eq!(Operand::Divide.perform(6, 5), 1);
    }

    #[test]
    fn monkey_process_items() {
        let mut monkey = Monkey {
            id: 0,
            items: vec![79, 98],
            operation: Operation { lhs: Value::Old, rhs: Value::Number(19), operand: Operand::Multiply },
            test: Test { test_div_by: 23, true_throw: 2, false_throw: 3 }
        };

        let result = monkey.process_items(&DivideWorry(3));
        assert_eq!(result, HashMap::from_iter(vec![
            (3, vec![500, 620])
        ]));
    }

    #[test]
    fn monkeys_play() {
        let input = include_str!("input.example.txt");
        let mut monkeys: Vec<Monkey> = input
            .split("Monkey ")
            .skip(1)
            .map(|monkey| monkey.parse().unwrap())
            .collect();
        let mut counts = vec![0 as usize; monkeys.len()];
        
        play(&mut monkeys, &mut counts, &DivideWorry(3));
        assert_eq!(
            monkeys[0].items,
            vec![20, 23, 27, 26]
        );
        assert_eq!(
            monkeys[1].items,
            vec![2080, 25, 167, 207, 401, 1046]
        );        
        assert_eq!(
            monkeys[2].items,
            vec![]
        );
        assert_eq!(
            monkeys[3].items,
            vec![]
        );

        play(&mut monkeys, &mut counts, &DivideWorry(3));
        assert_eq!(
            monkeys[0].items,
            vec![695, 10, 71, 135, 350]
        );
        assert_eq!(
            monkeys[1].items,
            vec![43, 49, 58, 55, 362]
        );        
        assert_eq!(
            monkeys[2].items,
            vec![]
        );
        assert_eq!(
            monkeys[3].items,
            vec![]
        );
    }

    #[test]
    fn part1_example() {
        let input = include_str!("input.example.txt");
        let result = part1(input);
        assert_eq!(result, 10605);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input.example.txt");
        let result = part2(input);
        assert_eq!(result, 2713310158);
    }

}