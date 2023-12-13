use std::{collections::{HashMap}, time::Instant, str::FromStr};

use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    txt.lines()
        .map(|l| l.parse::<Springs>().unwrap().check())
        .sum()
}

fn part2(txt: &str) -> usize {
    txt.lines()
    .map(|l| l.parse::<ExpandedSprings>().unwrap().check())
    .sum()
}

#[derive(Debug, Clone)]
struct Springs {
    puzzle: String,
    counts: Vec<usize>,
}

struct ExpandedSprings {
    puzzle: String,
    counts: Vec<usize>,
}

impl Springs {
    fn check(self) -> usize {
        let mut memo = HashMap::new();
        check(self.puzzle, &self.counts[0..], &mut memo)
    }
}

fn check<'a>(
    str: String,
    n: &'a [usize],
    memo: &mut HashMap<(String, &'a [usize]), usize>
) -> usize {
    
    if let Some(&result) = memo.get(&(str.clone(), n)) {
        return result;
    } 

    if n.is_empty() {
        // println!("** here empty");
        if str.contains("#") {
            memo.insert((str, n), 0);
            return 0;
        }
        memo.insert((str, n), 1);
        return 1;
    }
    if str.is_empty() {
        // println!("here empty empty");
        memo.insert((str, n), 0);
        return 0;
    }

    if &str[0..1] == "." {
        // println!("here dot");
        return check(str[1..].to_owned(), n, memo);
    }

    if &str[0..1] == "?" {
        let mut count = 0;
        let copy = str.clone();
        let copy = ".".to_owned() + &copy[1..];
        count += check(copy, n, memo);
        let copy = str.clone();
        let copy = "#".to_owned() + &copy[1..];
        count += check(copy, n, memo);

        memo.insert((str, n), count);
        return count;
    }

    let str_len = str.len();

    if &str[0..1] == "#" && str_len >= n[0] {
        let left = &str[0..n[0]];
        if left.contains(&".") {
            // println!("here contains dot [{}]", n[0]);
            memo.insert((str, n), 0);
            return 0;
        }

        if str_len == n[0] && n.len() == 1 {
            // println!("** here len match 1");
            memo.insert((str, n), 1);
            return 1;
        }

        let right = &str[n[0]..];

        if right.is_empty() {
            if n.len() == 1 {
                // println!("** here len match 2");
                memo.insert((str, n), 1);
                return 1;
            } else {
                memo.insert((str, n), 0);
                return 0;
            }
        }

        if !right.is_empty() && &right[0..1] == "#" {
            // println!("here right no dot {right:?}");
            // return check(str[1..].to_owned(), n);
            memo.insert((str, n), 0);
            return 0;
        }

        // println!("here taken '{right:?}'");
        let taken_count = check(
            right[1..].to_owned(), 
            &n[1..],
            memo
        );
        // println!("here not");
        // let not_taken_count = check(str[1..].to_owned(), n);
        memo.insert((str, n), taken_count);
        return taken_count;
    }

    // println!("hmm forgotten: {:?} and {:?}", str, n);
    memo.insert((str, n), 0);
    0
}

impl FromStr for Springs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let puzzle = parts.next().expect("left").to_owned();
        let count = parts.next().expect("right")
            .split(",")
            .map(|n| n.parse::<usize>().expect(format!("bad num: '{}'", n).as_str()))
            .collect();
        Ok(Springs { puzzle, counts: count })
    }
}

impl FromStr for ExpandedSprings {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let puzzle = parts.next().expect("left").to_owned();
        let count = parts.next().expect("right")
            .split(",")
            .map(|n| n.parse::<usize>().expect(format!("bad num: '{}'", n).as_str()))
            .collect_vec();

        let puzzle = format!(
            "{}?{}?{}?{}?{}",
            puzzle,
            puzzle,
            puzzle,
            puzzle,
            puzzle,
        );
        let mut count_mod = vec![];
        count_mod.extend(count.clone());
        count_mod.extend(count.clone());
        count_mod.extend(count.clone());
        count_mod.extend(count.clone());
        count_mod.extend(count);

        Ok(ExpandedSprings { puzzle, counts: count_mod })
    }
}

impl ExpandedSprings {
    fn check(self) -> usize {
        let mut memo = HashMap::new();
        check(self.puzzle, &self.counts[0..], &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(21, part1(include_str!("input.test.txt")));
    }


    #[test]
    fn test_example_p2() {
        assert_eq!(525152, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let mut s = include_str!("input.test.txt").lines()
            .map(|l| l.parse::<Springs>().unwrap())
            .collect_vec();
        let Springs{ puzzle: _, counts: count,} = s.pop().unwrap();
        assert_eq!(vec![3, 2, 1], count)
    }

    #[test]
    fn test_simple_examples_1() {
        let s: Springs = "? 1".parse().unwrap();
        let result = s.check();
        assert_eq!(1, result);
    }

    #[test]
    fn test_simple_examples_2() { 
        let s: Springs = "?? 1".parse().unwrap();
        let result = s.check();
        assert_eq!(2, result);
    }

    #[test]
    fn test_simple_examples_3() {
        let s: Springs = "??? 1".parse().unwrap();
        let result = s.check();
        assert_eq!(3, result);
    }

    #[test]
    fn test_simple_examples_4() {
        let s: Springs = "?.? 1".parse().unwrap();
        let result = s.check();
        assert_eq!(2, result);
    }

    #[test]
    fn test_recursion_p1_1() {
        let s: Springs = "???.### 1,1,3".parse().unwrap();
        let result = s.check();
        assert_eq!(1, result);
    }

    #[test]
    fn test_recursion_p1_2() {
        let s: Springs = ".??..??...?##. 1,1,3".parse().unwrap();
        let result = s.check();
        assert_eq!(4, result);
    }

    #[test]
    fn test_recursion_p1_3() {
        let s: Springs = "?#?#?#?#?#?#?#? 1,3,1,6".parse().unwrap();
        let result = s.check();
        assert_eq!(1, result);
    }

    #[test]
    fn test_recursion_p1_4() {
        let s: Springs = "????.#...#... 4,1,1".parse().unwrap();
        let result = s.check();
        assert_eq!(1, result);
    }

    #[test]
    fn test_recursion_p1_5() {
        let s: Springs = "????.######..#####. 1,6,5".parse().unwrap();
        let result = s.check();
        assert_eq!(4, result);
    }

    #[test]
    fn test_recursion_p1_6() {
        let s: Springs = "?###???????? 3,2,1".parse().unwrap();
        let result = s.check();
        assert_eq!(10, result);
    }

    #[test]
    fn test_trailing_hash() {
        let s: Springs = "?...# 1".parse().unwrap();
        let result = s.check();
        assert_eq!(1, result);
    }
}
