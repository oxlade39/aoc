use core::panic;
use std::{time::Instant, collections::HashSet, hash::Hash, fmt::Debug, iter::repeat};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.txt");

    let image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    let input_image = InputImage::new( input.lines().skip(2).collect() );

    let mut n = next(&input_image, &image_enhancement_algo);
    println!("");
    println!("{:?}", n);
    println!("");
    n = next(&n, &image_enhancement_algo);

    println!("");
    println!("{:?}", n);
    println!("");

    println!("{} pixels", n.light_pixels.len());
}

fn part2() {
    let input = include_str!("input.txt");

    let image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    let input_image = InputImage::new( input.lines().skip(2).collect() );

    let result = repeat(()).take(50)
        .fold(input_image, |input, _| next(&input, &image_enhancement_algo));
    println!("Part2 Image:\n{:?}", result);
    println!("Part2:\n{:?}", result.light_pixels.len());
}

fn bin_to_i16(s: &str) -> i16 {
    let err = format!("bad input: {}", s);
    i16::from_str_radix(s, 2).expect(&err)
}

fn next(input_image: &InputImage, image_enhancement_algo: &ImageEnhancementBits) -> InputImage {
    let mut new_input_image: HashSet<Position> = HashSet::new();

    // if current infinity is light then next infinity will be 512, else 0
    let current_infinity_light = input_image.light_pixels.contains(&Position{ x: 0, y: 0 });
    let next_infinity_light = if current_infinity_light {
        image_enhancement_algo.max_light()
    } else {
        image_enhancement_algo.zero_light()
    };

    for row in 0..(input_image.height + 3) {
        for col in 0..(input_image.width + 3) {
            if row < 2 || col < 2 || row > input_image.height || col > input_image.width {
                if next_infinity_light {
                    let p = Position{ x: col, y: row };
                    new_input_image.insert(p);
                }
            } else {
                // shift pixels down and right each step
                let p = Position{ x: col - 1, y: row - 1 };
                let next_p = Position{ x: col, y: row };
                let result = input_image.is_light(&p, &image_enhancement_algo);
                if result {
                    new_input_image.insert(next_p);
                }
            }
        }
    }

    InputImage{ light_pixels: new_input_image, width: input_image.width + 2, height: input_image.height + 2 }
}

#[derive(Clone, PartialEq, Eq)]
struct InputImage {
    light_pixels: HashSet<Position>,
    width: i64,
    height: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ImageEnhancementBits(HashSet<i64>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        vec![
            Position{ x: self.x - 1, y: self.y - 1}, 
            Position{ x: self.x, y: self.y - 1}, 
            Position{ x: self.x + 1, y: self.y - 1}, 

            Position{ x: self.x - 1, y: self.y }, 
            Position{ x: self.x, y: self.y }, 
            Position{ x: self.x + 1, y: self.y }, 

            Position{ x: self.x - 1, y: self.y + 1 }, 
            Position{ x: self.x, y: self.y +1 }, 
            Position{ x: self.x + 1, y: self.y + 1 }, 
        ]
    }

}

impl InputImage {
    fn new(s: Vec<&str>) -> InputImage {
        let mut light_pixels: HashSet<Position> = HashSet::new();
        for (row_num, row) in s.iter().enumerate() {
            for (col_num, c) in row.chars().enumerate() {
                if c == '#' {
                    let x = (col_num + 2) as i64;
                    let y = (row_num + 2) as i64;
                    light_pixels.insert(Position{ x, y });
                }
            }
        }
        let width = (s.iter().next().map(|line| line.len()).unwrap_or(0) + 4) as i64;
        let height = (s.len() + 4) as i64;
        InputImage{ light_pixels, width, height }
    }

    fn is_light(&self, p: &Position, iha: &ImageEnhancementBits) -> bool {
        let it: String = p.neighbours().iter().map(|p| self.pixel_str(p)).collect();
        let index = bin_to_i16(&it) as i64;
        iha.0.contains(&index)
    }

    fn pixel_str(&self, p: &Position) -> &str {
        if self.light_pixels.contains(&Position{ x: p.x, y: p.y}) {
            "1"
        } else {
            "0"
        }
    }
}

impl Debug for InputImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Ok(());
        for i in 0..self.height {
            for j in 0..self.width {
                let col = if self.light_pixels.contains(&Position{ x: j as i64, y: i as i64 }) {
                    "#"
                } else {
                    "."
                };
                result = f.write_str(col);
                if result.is_err() {
                    return result;
                }
            }
            result = f.write_str("\n");
        }
        result
    }
}

impl ImageEnhancementBits {
    fn new(s: &str) -> ImageEnhancementBits {
        if s.len() != 512 {
            panic!("image enhancement bits were {} long, expected 512", s.len());
        }

        let mut bits = HashSet::new();

        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                bits.insert(i as i64);
            }
        }

        ImageEnhancementBits(bits)
    }

    fn zero_light(&self) -> bool {
        self.0.contains(&0)
    }

    fn max_light(&self) -> bool {
        self.0.contains(&511)
    }
}

#[test]
fn test_bin() {
    let example = "000100010";
    let result = bin_to_i16(example);
    assert_eq!(34, result);
}

#[test]
fn test_parse_input_image() {
    let input = include_str!("input.test.txt");
    let input_image = InputImage::new( input.lines().skip(2).collect() );

    println!("input:\n{:?}", input_image);

    assert_eq!(10, input_image.light_pixels.len());
}

#[test]
fn test_is_light_false_zero() {
    let ii = InputImage { 
        light_pixels: HashSet::from_iter(vec![
            Position{ x: 3, y: 3 }
        ]),
        height: 4,
        width: 4,
    };
    let iha = ImageEnhancementBits(HashSet::new());

    println!("ii:\n{:?}", ii);

    assert_eq!(false, ii.is_light(&Position { x: 0, y: 0 }, &iha))
}

#[test]
fn test_is_light_true_zero() {
    let ii = InputImage { 
        light_pixels: HashSet::from_iter(vec![]),
        height: 3,
        width: 3,
    };
    let iha = ImageEnhancementBits(HashSet::from_iter(vec![0]));

    println!("ii:\n{:?}", ii);

    assert_eq!(true, ii.is_light(&Position { x: 1, y: 1 }, &iha))
}

#[test]
fn test_is_light_true_max() {
    let ii = InputImage { 
        light_pixels: HashSet::from_iter(vec![
            Position{ x: 0, y: 0 },
            Position{ x: 1, y: 0 },
            Position{ x: 2, y: 0 },
            Position{ x: 0, y: 1 },
            Position{ x: 1, y: 1 },
            Position{ x: 2, y: 1 },
            Position{ x: 0, y: 2 },
            Position{ x: 1, y: 2 },
            Position{ x: 2, y: 2 },
        ]),
        height: 3,
        width: 3,
    };
    let iha = ImageEnhancementBits(HashSet::from_iter(vec![511]));

    println!("ii:\n{:?}", ii);

    assert_eq!(true, ii.is_light(&Position { x: 1, y: 1 }, &iha))
}

#[test]
fn test_parse_ieb() {
    let input = include_str!("input.test.txt");

    let image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    assert_eq!(image_enhancement_algo.0.contains(&34), true);
    assert_eq!(image_enhancement_algo.0.contains(&70), false);
}

#[test]
fn test_correct_count() {
    let mut input = include_str!("input.test.txt");

    let mut image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    let mut input_image = InputImage::new( input.lines().skip(2).collect() );

    let mut result = repeat(()).take(2)
        .fold(input_image, |input, _| next(&input, &image_enhancement_algo));
    println!("35:\n{:?}", result);
    assert_eq!(result.light_pixels.len(), 35);

    input = include_str!("input.txt");

    image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    input_image = InputImage::new( input.lines().skip(2).collect() );

    result = repeat(()).take(2)
        .fold(input_image, |input, _| next(&input, &image_enhancement_algo));
    println!("5583:\n{:?}", result);
    assert_eq!(result.light_pixels.len(), 5583);

    input = include_str!("input.test.txt");

    image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    input_image = InputImage::new( input.lines().skip(2).collect() );

    result = repeat(()).take(50)
        .fold(input_image, |input, _| next(&input, &image_enhancement_algo));
    println!("3351:\n{:?}", result);
    assert_eq!(result.light_pixels.len(), 3351);
}