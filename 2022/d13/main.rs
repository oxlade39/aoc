use std::{str::FromStr, fmt::Debug};

use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    println!("part1: {}", part1);
    let part2 = part2(input);
    println!("part2: {}", part2);
}

#[derive(Debug, PartialEq)]
enum Order {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(PartialEq, Eq, Clone)]
enum PacketItem {
    SimplePacketItem(i64),
    ComplexPacketItem(Vec<PacketItem>),
}

impl FromStr for PacketItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = parse(s, 0, vec![vec![]]);
        result
            .pop()
            .and_then(|mut v| v.pop())
            .ok_or("no value".to_string())
    }
}

impl Debug for PacketItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SimplePacketItem(i) => f.write_fmt(format_args!("{}", i)),
            Self::ComplexPacketItem(children) => f.write_fmt(format_args!("{:?}", children)),
        }
    }
}

impl PacketItem {
    fn order(&self, right: &Self) -> Order {
        // println!("comparing {:?} vs {:?}", self, right);
        match self {
            PacketItem::SimplePacketItem(left_i) => match right {
                PacketItem::SimplePacketItem(right_i) => {
                    if left_i < right_i {
                        Order::Correct
                    } else if left_i == right_i {
                        Order::Unknown
                    } else {
                        Order::Incorrect
                    }
                },
                PacketItem::ComplexPacketItem(_) => {
                    let as_complex = PacketItem::ComplexPacketItem(vec![
                        PacketItem::SimplePacketItem(*left_i)
                    ]);
                    as_complex.order(right)
                }
            },
            PacketItem::ComplexPacketItem(left_children) => match right {
                PacketItem::SimplePacketItem(right_i) => {
                    let as_complex = PacketItem::ComplexPacketItem(vec![
                        PacketItem::SimplePacketItem(*right_i)
                    ]);
                    self.order(&as_complex)
                },
                PacketItem::ComplexPacketItem(right_children) => {
                    let mut left_itr = left_children.iter();
                    let mut right_itr = right_children.iter();

                    loop {
                        let next_left = left_itr.next();
                        let next_right = right_itr.next();
                        
                        match (next_left, next_right) {
                            (None, None) => return Order::Unknown,
                            (None, _) => return Order::Correct,
                            (_, None) => return Order::Incorrect,
                            (Some(left), Some(right)) => {
                                let next_order = left.order(right);
                                if next_order != Order::Unknown {
                                    return next_order;
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

fn parse(s: &str, index: usize, mut stack: Vec<Vec<PacketItem>>) -> Vec<Vec<PacketItem>> {
    if index >= s.len() {        
        if s != "" {        
            let pi = PacketItem::SimplePacketItem(s.parse().expect(format!("{:?}", s).as_str()));
            let mut top = stack.pop().unwrap_or(vec![]);
            top.push(pi);
            stack.push(top);
        };        
        return stack;
    }
    
    let current_str = &s[index..index+1];
    if current_str == "[" {
        stack.push(vec![]);
        parse(&s[index+1..], 0, stack)
    } else if current_str == "]" {
        let left = &s[..index];
        let right = &s[index + 1..];
        // println!("] with {:?} - {:?} - d{}", left, right, stack.len());
        let mut left_items = parse(left, 0, stack);
        let it = left_items.pop().map(PacketItem::ComplexPacketItem);
        let mut top = left_items.pop().unwrap();
        top.push(it.unwrap());
        left_items.push(top);
        parse(right, 0, left_items)
    } else if current_str == "," {
        let left = &s[..index];
        let right = &s[index + 1..];
        // println!(", with {:?} - {:?} - d{}", left, right, stack.len());
        let left_items = parse(left, 0, stack);
        parse(right, 0, left_items)
    } else {
        parse(s, index + 1, stack)
    }
}

fn part1(input: &str) -> i64 {
    let items: Vec<Vec<PacketItem>> = input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|c| {
                c.take(2)
                .map(|item| item.parse().unwrap())
                .collect::<Vec<_>>()
            })
            .collect();
    for pair in &items {
        println!("{:?}", pair[0]);
        println!("{:?}", pair[1]);
        println!();
    }
    items
        .into_iter()
        .enumerate()
        .filter_map(|(i, packets)| {
            let order = packets[0].order(&packets[1]);
            match order {
                Order::Correct => Some(i as i64 + 1),
                _ => None,
            }
        })
        .sum()
}

impl From<Order> for std::cmp::Ordering {
    fn from(value: Order) -> Self {
        match value {
            Order::Correct => Self::Less,
            Order::Incorrect => Self::Greater,
            Order::Unknown => Self::Equal,
        }
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order(other).into())
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order(other).into()
    }
}

fn part2(input: &str) -> usize {
    let divider_packets: Vec<PacketItem> = vec![
        "[[2]]".parse().unwrap(),
        "[[6]]".parse().unwrap(),
    ];

    let mut items = divider_packets.clone();
    let mut input_packets: Vec<_> = input.lines()
        .filter_map(|line| line.parse::<PacketItem>().ok())
        .collect();
    items.append(&mut input_packets);
    items.sort();
    
    divider_packets
        .iter()
        .map(|p| {
            items.iter().position(|item| item == p).unwrap() + 1
        })
        .product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_base_case() {
        let result: PacketItem = "[]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![]));
    }

    #[test]
    fn test_parse_single_item() {
        let result: PacketItem = "[1]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![
            PacketItem::SimplePacketItem(1),
        ]));
    }

    #[test]
    fn test_parse_two_items() {
        let result: PacketItem = "[1,2]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![
            PacketItem::SimplePacketItem(1),
            PacketItem::SimplePacketItem(2),
        ]));
    }

    #[test]
    fn test_parse_three_items() {
        let result: PacketItem = "[1,2,3]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![
            PacketItem::SimplePacketItem(1),
            PacketItem::SimplePacketItem(2),
            PacketItem::SimplePacketItem(3),
        ]))
    }

    #[test]
    fn test_parse_single_nest_items() {
        let result: PacketItem = "[1,2,[3]]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![
            PacketItem::SimplePacketItem(1),
            PacketItem::SimplePacketItem(2),
            PacketItem::ComplexPacketItem(vec![
                PacketItem::SimplePacketItem(3)
            ]),
        ]))
    }

    #[test]
    fn test_parse_more_complex_nested() {
        let result: PacketItem = "[[1],[2,3,4]]".parse().unwrap();
        
        assert_eq!(result, PacketItem::ComplexPacketItem(vec![
            PacketItem::ComplexPacketItem(vec![
                PacketItem::SimplePacketItem(1),
            ]),
            PacketItem::ComplexPacketItem(vec![
                PacketItem::SimplePacketItem(2),
                PacketItem::SimplePacketItem(3),
                PacketItem::SimplePacketItem(4),
            ]),
        ]));
    }

    #[test]
    fn test_compare_example_1() {
        // [1,1,3,1,1] vs [1,1,5,1,1]
        let left: PacketItem = "[1,1,3,1,1]".parse().unwrap();
        let right: PacketItem = "[1,1,5,1,1]".parse().unwrap();
        let order = left.order(&right);
        assert_eq!(Order::Correct, order);
    }

    #[test]
    fn test_compare_example_2() {
        // [[1],[2,3,4]] vs [[1],4]
        let left: PacketItem = "[[1],[2,3,4]]".parse().unwrap();
        let right: PacketItem = "[[1],4]".parse().unwrap();
        let order = left.order(&right);
        assert_eq!(Order::Correct, order);
    }

    #[test]
    fn test_compare_example_3() {
        // [9] vs [[8,7,6]]
        let left: PacketItem = "[9]".parse().unwrap();
        let right: PacketItem = "[[8,7,6]]".parse().unwrap();
        let order = left.order(&right);
        assert_eq!(Order::Incorrect, order);
    }

    #[test]
    fn test_compare_example_4() {
        // [[4,4],4,4] vs [[4,4],4,4,4]
        let left: PacketItem = "[[4,4],4,4]".parse().unwrap();
        let right: PacketItem = "[[4,4],4,4,4]".parse().unwrap();
        let order = left.order(&right);
        assert_eq!(Order::Correct, order);
    }

    #[test]
    fn test_compare_example_5() {
        // [7,7,7,7] vs [7,7,7]
        let left: PacketItem = "[7,7,7,7]".parse().unwrap();
        let right: PacketItem = "[7,7,7]".parse().unwrap();
        let order = left.order(&right);
        assert_eq!(Order::Incorrect, order);
    }

    #[test]
    fn test_parse_example() {
        let input = include_str!("input.example.txt");
        let result = part1(input);
        assert_eq!(13, result);
    }

    #[test]
    fn test_parse_my_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(6272, result);
    }

    #[test]
    fn test_part_2_example() {
        let input = include_str!("input.example.txt");
        let result = part2(input);
        assert_eq!(140, result);
    }

}