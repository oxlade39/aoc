
fn main(){
    let input = include_str!("input.txt");
    let result = max(input);
    println!("max: {result}")
}

fn max(input: &str) -> i32 {
    let mut max  = 0;
    let mut current = 0;
    for line in input.lines() {
        if let Ok(i) = line.parse::<i32>() {
            current += i;
        } else {
            if current > max {
                max = current;
            }
            current = 0;
        }
    }
    max
}

#[test]
fn test_example(){
    let input = include_str!("input.test.txt");
    assert_eq!(24000, max(input));
}