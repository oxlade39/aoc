use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WIDTH: usize = 12;

fn main() -> io::Result<()>{
    let lines = read_lines("./input/3.a.txt")?;
    
    let mut bits: [i32; WIDTH] = [0; WIDTH];
    let mut count: i32 = 0;

    for line in lines {
        let item = line.unwrap();
        let split: Vec<_> = item
            .split("")
            .filter_map(|char| char.parse::<i32>().ok())
            .collect();
        for i in 0..WIDTH {            
            bits[i] += split[i]
        }
        count += 1;
    }
    println!("bits: {:?}", bits);
    println!("count: {:?}", count);
    let hurdle = count / 2;
    let mut g_total = 0;
    let mut e_total = 0;

    for i in 0..WIDTH {
        let pos = (WIDTH-1) - i;
        let bit = bits[pos];
        if bit > hurdle {
            g_total = g_total + 2_i32.pow(i.try_into().unwrap())
        } else {
            e_total = e_total + 2_i32.pow(i.try_into().unwrap())
        };        
    }

    println!("result: g:{}, e:{} {:?}", g_total, e_total, g_total * e_total);

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}