use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() -> io::Result<()>{
    let lines = read_lines("./input/3.b.txt")?;
    
    let mut rows: Vec<Vec<i32>> = Vec::new();

    let mut col_sums: Vec<i32> = Vec::new();

    for (i, line) in lines.enumerate() {
        let item = line.unwrap();
        let mut row: Vec<i32> = Vec::with_capacity(item.len());
        if i == 0 {
            col_sums.resize(item.len(), 0);
        }

        for (j, c) in item.chars().enumerate() {
            let as_i = c.to_digit(10).unwrap().try_into().unwrap();
            row.push(as_i);
            col_sums[j] += as_i;
        }
        rows.push(row);
    }
    
    println!("col_sums {:?}", col_sums);

    let mut o2: Vec<&Vec<i32>> = Vec::with_capacity(rows.len());
    for row in &rows {
        o2.push(row);
    }

    for col_idx in 0..col_sums.len() {

        let current_len = o2.len();
        if current_len == 1 {
            continue;
        }
        let mut o2_filtered: Vec<&Vec<i32>> = Vec::with_capacity(current_len);

        let col_count: i32 = o2.iter()
            .map(|row| row[col_idx])
            .filter(|i| i > &0)
            .count()
            .try_into()
            .unwrap();

        let total: i32 = current_len.try_into().unwrap();
        let hurdle: i32 = total / 2 + total % 2;

        let most_common = if col_count >= hurdle {
            1
        } else {
            0
        };

        for row in o2 {
            if row[col_idx] == most_common {
                o2_filtered.push(row);
            }
        }

        o2 = o2_filtered;
    }

    let mut co2: Vec<&Vec<i32>> = Vec::with_capacity(rows.len());
    for row in &rows {
        co2.push(row);
    }

    for col_idx in 0..col_sums.len() {

        let current_len = co2.len();
        if current_len == 1 {
            continue;
        }

        let mut co2_filtered: Vec<&Vec<i32>> = Vec::with_capacity(current_len);

        let col_count: i32 = co2.iter()
            .map(|row| row[col_idx])
            .filter(|i| i > &0)
            .count()
            .try_into()
            .unwrap();
        let total: i32 = current_len.try_into().unwrap();
        let hurdle: i32 = total / 2 + total % 2;

        let least_common = if col_count >= hurdle {
            0
        } else {
            1
        };

        for row in co2 {
            if row[col_idx] == least_common {
                co2_filtered.push(row);
            }
        }

        co2 = co2_filtered;
    }

    println!("o2: {:?}", o2);
    println!("co2: {:?}", co2);

    
    println!("result = {} * {} = {}", sum(o2[0]), sum(co2[0]), sum(o2[0]) * sum(co2[0]));

    Ok(())
}

fn sum(v: &Vec<i32>) -> i32 {
    let mut count = 0;
    let len = v.len();
    for i in 0..v.len() {
        let ex = len - (i + 1);
        count += i32::pow(2, ex.try_into().unwrap()) * v[i];
    }
    return count;
}

#[test]
fn test_sum() {
    assert_eq!(sum(vec![0, 0, 0, 0, 1]), 1);
    assert_eq!(sum(vec![0, 1, 0, 1, 0]), 10);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}