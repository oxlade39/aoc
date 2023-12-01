use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
}

fn part1(txt: &str) -> i32 {
    const RADIX: u32 = 10;

    txt.lines()
        .map(|l| l.chars()
            .filter(|c| c.to_digit(RADIX).is_some())
            .collect_vec()
        )
        .map(|l| format!("{}{}", l.first().unwrap(), l.last().unwrap()))
        .map(|i| i.parse::<i32>().unwrap())
        .sum()
}


#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn sample_input_pt1() {
        let input = include_str!("input.test.txt");
        assert_eq!(142, part1(input));
    }
}