use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let parsed: Input = input.parse().unwrap();
    let mut stacks = parsed.0;
    for m in parsed.1 {
        stacks.apply(&m);
    }
    let str: String = stacks.top().into_iter().map(|c| c.0).collect();
    println!("part1: {:?}", str);
}

#[derive(Debug, PartialEq)]
struct Stacks {
    crates: Vec<Vec<Crate>>
}

#[derive(Debug, PartialEq)]
struct Crate(char);

#[derive(Debug, PartialEq)]
struct Move {
    crate_count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq)]
struct Input(Stacks, Vec<Move>);

#[derive(Debug, PartialEq)]
enum InputError {
}

impl FromStr for Input {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stacks: Vec<_> = s
            .lines()
            .take_while(|line| !line.is_empty())            
            .collect();
        let moves: Vec<_> = s
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                let parts: Vec<_> = line.split(" ").collect();
                Move { 
                    crate_count: parts[1].parse().unwrap(), 
                    from: parts[3].parse().unwrap(), 
                    to: parts[5].parse().unwrap()
                }
            })
            .collect();

        let stack_counts = stacks[stacks.len() - 1];
        let stack_count: u32 = stack_counts
            .chars()
            .nth(stack_counts.len() - 2)
            .unwrap()
            .to_digit(10)
            .unwrap();

        let mut parsed_stacks: Vec<Vec<Crate>> = Vec::new();
        for width in 0..stack_count {
            let mut width_stack: Vec<Crate> = Vec::new();
            let width_pos = (width as usize * 4) + 1;
            for height in (0..(stacks.len()-1)).rev() {                
                let stack_line = stacks[height];
                let stack_code = stack_line.chars().nth(width_pos).unwrap();
                match stack_code {
                    ' ' => {},
                    other => width_stack.push(Crate(other)),
                };
            }
            parsed_stacks.push(width_stack);
        }
        Ok(Input(Stacks { crates: parsed_stacks }, moves))
    }
}

impl Stacks {
    fn apply(& mut self, to_apply: &Move) {
        for _ in 0..to_apply.crate_count {
            let to_move = self.crates[to_apply.from - 1].pop().unwrap();
            self.crates[to_apply.to - 1].push(to_move);
        }
    }

    fn top(&self) -> Vec<&Crate> {
        let mut top: Vec<_> = Vec::new();
        for stack in &self.crates {
            let item = stack.last().unwrap();
            top.push(item);
        }
        top
    }
}

#[test]
fn test_parse_example() {
    let input = include_str!("input.example.txt");
    let parsed: Input = input.parse().unwrap();

    assert_eq!(Input(Stacks { crates: vec![
        vec![Crate('Z'), Crate('N')], 
        vec![Crate('M'), Crate('C'), Crate('D')], 
        vec![Crate('P')]
    ] }, vec![
        Move { crate_count: 1, from: 2, to: 1 },
        Move { crate_count: 3, from: 1, to: 3 },
        Move { crate_count: 2, from: 2, to: 1 },
        Move { crate_count: 1, from: 1, to: 2 },
    ]), parsed);
}

#[test]
fn apply_move() {
    let mut stacks = Stacks { crates: vec![
        vec![Crate('Z'), Crate('N')], 
        vec![Crate('M'), Crate('C'), Crate('D')], 
        vec![Crate('P')]
    ] };
    let m = Move { crate_count: 1, from: 2, to: 1 };
    stacks.apply(&m);
    assert_eq!(Stacks { crates: vec![
        vec![Crate('Z'), Crate('N'), Crate('D')], 
        vec![Crate('M'), Crate('C')], 
        vec![Crate('P')]
    ] }, stacks);
}

#[test]
fn test_part_1_example() {
    let input = include_str!("input.example.txt");
    let parsed: Input = input.parse().unwrap();

    let mut stacks = parsed.0;
    for m in parsed.1 {
        stacks.apply(&m);
    }
    let top = stacks.top();
    assert_eq!(vec![&Crate('C'), &Crate('M'), &Crate('Z')], top);
}