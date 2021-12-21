use std::{time::Instant, collections::HashSet, hash::Hash, fmt::{Debug, Pointer}};

fn main() {
    let start = Instant::now();
    part1();
    part2();
    println!("took: {} ms", start.elapsed().as_millis())
}

fn part1() {
    let input = include_str!("input.test.txt");

    let image_enhancement_algo = ImageEnhancementBits::new(input.lines().next().unwrap());
    let input_image = InputImage::new( input.lines().skip(2).collect() ).re_center();

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
}

fn bin_to_i16(s: &str) -> i16 {
    let err = format!("bad input: {}", s);
    i16::from_str_radix(s, 2).expect(&err)
}

fn bin_to_i64(s: &str) -> i64 {
    i64::from_str_radix(s, 2).unwrap()
}

fn bin_to_usize(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn next(input_image: &InputImage, image_enhancement_algo: &ImageEnhancementBits) -> InputImage {
    let mut new_input_image: HashSet<Position> = HashSet::new();

    for row in -3..(input_image.bottom() + 3) {
        for col in -3..(input_image.right() + 3) {
            let p = Position{ x: col, y: row };
            let result = input_image.is_light(&p, &image_enhancement_algo);
            if result {
                new_input_image.insert(p);
            }
        }
    }

    InputImage{ light_pixels: new_input_image }.re_center()
}

#[derive(Clone, PartialEq, Eq)]
struct InputImage {
    light_pixels: HashSet<Position>
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
                    let x = col_num as i64;
                    let y = row_num as i64;
                    light_pixels.insert(Position{ x, y });
                }
            }
        }
        InputImage{ light_pixels }
    }

    fn is_light(&self, p: &Position, iha: &ImageEnhancementBits) -> bool {
        let it: String = p.neighbours().iter().map(|p| self.pixel_str(p)).collect();
        let index = bin_to_i16(&it) as i64;
        iha.0.contains(&index)
    }

    fn pixel_str(&self, p: &Position) -> &str {
        if self.light_pixels.contains(&Position{ x: p.x - 1, y: p.y - 1}) {
            "1"
        } else {
            "0"
        }
    }

    fn bottom(&self) -> i64 {
        self.light_pixels.iter().map(|p| p.y).max().unwrap()
    }

    
    fn right(&self) -> i64 {
        self.light_pixels.iter().map(|p| p.x).max().unwrap()
    }

    fn center(&self, p: &Position) -> Self {
        let mut moved: HashSet<_> = HashSet::new();
        for pos in self.light_pixels.iter() {
            moved.insert(Position {
                x: (pos.x - p.x),
                y: (pos.y - p.y),
            });
        }

        Self {
            light_pixels: moved
        }
    }

    fn re_center(&self) -> Self {
        let min_x = self.light_pixels.iter().map(|p| p.x).min().unwrap();
        let min_y = self.light_pixels.iter().map(|p| p.y).min().unwrap();
        self.center(&Position{ x: min_x, y: min_y })
    }
}

impl Debug for InputImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..=self.bottom() {
            for j in 0..=self.right() {
                let col = if self.light_pixels.contains(&Position{ x: j as i64, y: i as i64 }) {
                    "#"
                } else {
                    "."
                };
                f.write_str(col);
            }
            f.write_str("\n");
        }
        Ok(())
    }
}

impl ImageEnhancementBits {
    fn new(s: &str) -> ImageEnhancementBits {
        let mut bits = HashSet::new();

        for (i, c) in s.chars().enumerate() {
            if c == '#' {
                bits.insert(i as i64);
            }
        }

        ImageEnhancementBits(bits)
    }
}

#[test]
fn test_center() {
    let example = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: 0, y: 0 }
    ])};

    let result = example.center(&Position{ x: -1, y: -1 });
    let expected = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: 1, y: 1 }
    ])};
    assert_eq!(expected, result);
}

#[test]
fn test_recenter() {
    let example = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: 1, y: 1 }
    ])};

    let result = example.re_center();
    let expected = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: 0, y: 0 }
    ])};
    println!("bottom: {:?}", expected.bottom());
    println!("expected:\n{:?}", expected);
    println!("result:\n{:?}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_recenter_negative() {
    let example = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: -2, y: -2 },
        Position{ x: 0, y: 0 },
    ])};
    println!("before:\n{:?}", example);

    let result = example.re_center();
    let expected = InputImage { light_pixels: HashSet::from_iter(vec![
        Position{ x: 0, y: 0 },
        Position{ x: 2, y: 2 },
    ])};
    println!("bottom: {:?}", expected.bottom());
    println!("expected:\n{:?}", expected);
    println!("result:\n{:?}", result);
    assert_eq!(expected, result);
}

#[test]
fn test_bin() {
    let example = "000100010";
    let result = bin_to_i16(example);
    assert_eq!(34, result);
}