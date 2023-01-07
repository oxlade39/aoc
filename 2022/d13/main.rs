use std::{str::FromStr, fmt::Debug};

use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    println!("part1: {}", part1);
}

#[derive(Debug, PartialEq)]
enum Order {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(PartialEq)]
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
                        
                        if next_left == None {
                            return Order::Correct;
                        }

                        if next_right == None {
                            return Order::Incorrect;
                        }

                        let left = next_left.unwrap();
                        let right = next_right.unwrap();
                        let next_order = left.order(right);
                        if next_order != Order::Unknown {
                            return next_order;
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

fn part1_items(input: &str) -> Vec<i64> {
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
        .collect()
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


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
    fn test_bad_items() {
        let input = include_str!("input.txt");
        let items: Vec<Vec<&str>> = input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|c| {
                c.take(2)
                .collect::<Vec<_>>()
            })
            .collect();

        println!("{:?}", items[89]);
        println!();
        println!("{:?}", items[110]);

        let left: PacketItem = "[[],[7,6,[],[3,6,10,[9,7],4],[]],[4,9,8,[],2],[]]".parse().unwrap();
        let right: PacketItem = "[[],[4,[9,1,10,9,6],4,0],[[],[[],[10,7]],[[6,5],5,[3,2,6,3]],[2,3,10,10],[]],[9,[2,9]],[[5],10]]".parse().unwrap();
        let order = left.order(&right);
        println!("order: {:?}", order);
        println!("l: {:?}", left);
        println!("r: {:?}", right);


        let left: PacketItem = "[[],[[[7,6,1,0],6,7,7]],[6],[[],6]]".parse().unwrap();
        let right = "[[],[],[[],[[]],[[8,5],9,[2],8,5],4]]".parse().unwrap();
        let order = left.order(&right);
        println!("order: {:?}", order);
    }

    #[test]
    fn test_expected_pairs() {
        let input = include_str!("input.txt");
        let expected = vec![
            3,6,7,8,10,14,15,20,21,22,27,33,34,38,40,42,43,44,
            46,49,52,53,55,58,60,61,63,65,66,68,69,72,73,74,77,
            79,84,85,87,88,94,95,97,100,101,102,103,104,105,106,
            108,109,112,113,114,115, 117, 119, 120, 122, 123, 
            125, 126, 129, 131, 136, 138, 139, 140, 141, 143, 144, 145, 146, 149, 150
        ];
        let items = part1_items(input);
        println!("expected:\n{:?}", expected);
        println!("items:\n{:?}", items);

        let expected: HashSet<i64> = HashSet::from_iter(expected);
        let items = HashSet::from_iter(items);

        let diff: Vec<_> = items.difference(&expected).collect();
        let empty: Vec<i64> = vec![];
        let empty: Vec<_> = empty.iter().collect();

        // mine is giving these when it shouldn't
        // 90, 111
        assert_eq!(diff, empty);

        let sum: i64 = items.iter().sum();
        println!("sum: {:?}", sum);

        let sum: i64 = expected.iter().sum();
        println!("sum: {:?}", sum);
    }

    #[test]
    fn test_parse_my_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(6272, result);
    }

}