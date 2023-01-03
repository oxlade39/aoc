use std::str::FromStr;

use aoclib::{cartesian::{Point, Plane}, astar::{Cost, DirectNeighbours, self, StraightLine}};

fn main() {
    let input = include_str!("input.txt");
    let pt1 = part1(input);
    println!("pt1: {}", pt1);
}

fn part1(input: &str) -> usize {
    let parsed: HeightMap = input.parse().unwrap();
    let start = &parsed.start.clone();
    let end = &parsed.end.clone();
    let plane = parsed.to_plane();

    let shortest_path = astar::astar(
        start.clone(), 
        end.clone(), 
        &StraightLine, 
        &OnlyUpOneCost(&parsed, StraightLine), 
        &DirectNeighbours(&plane),
    ).unwrap();
    shortest_path.path.len()
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<i64>>,
    start: Point,
    end: Point,
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_grid: Vec<Vec<_>> = s.lines()
            .map(|l| l.chars().collect())
            .collect();
        let width = char_grid[0].len();
        let height = char_grid.len();
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;
        let mut map: Vec<Vec<i64>> = vec![vec![0; width]; height];

        for y in 0..height {
            for x in 0..width {
                let p: Point = (x as i64, y as i64).into();
                let char_x = x;
                let char_y = height - (y + 1);
                let c = char_grid[char_y][char_x];
                match c {
                    'S' => {
                        start = Some(p);
                        map[y][x] = 0;
                    },
                    'E' => {
                        end = Some(p);
                        map[y][x] = ('z' as i64) - ('a' as i64);
                    },
                    any_other => {
                        map[y][x] = (any_other as i64) - ('a' as i64);
                    }
                }
            } 
        }

        Ok(HeightMap { 
            start: start.ok_or_else(|| "start missing")?,
            end: end.ok_or_else(|| "end mising".to_string())?,
            map
        })
    }
}

impl HeightMap {
    fn height_at(&self, p: &Point) -> i64 {
        self.map[p.y as usize][p.x as usize]
    }

    fn to_plane(&self) -> Plane {
        (
            self.map[0].len() as i64,
            self.map.len() as i64
        ).into()
    }
}

struct OnlyUpOneCost<'a, T: Cost>(&'a HeightMap, T);

impl<T: Cost> Cost for OnlyUpOneCost<'_, T> {
    fn measure(&self, from: &Point, to: &Point) -> i64 {
        let from_height = self.0.height_at(from);
        let to_height = self.0.height_at(to);
        if (to_height - from_height) > 1 {
            10000
        } else if (to_height - from_height) == 1 {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use aoclib::astar::{self, StraightLine, DirectNeighbours};

    use crate::*;


    #[test]
    fn test_parse_input() {
        let input = include_str!("input.example.txt");
        let parsed: HeightMap = input.parse().unwrap();
        assert_eq!(vec![
            vec![0, 1, 3, 4, 5, 6, 7, 8], 
            vec![0, 2, 2, 19, 20, 21, 22, 9], 
            vec![0, 2, 2, 18, 25, 25, 23, 10], 
            vec![0, 1, 2, 17, 24, 23, 23, 11], 
            vec![0, 0, 1, 16, 15, 14, 13, 12]],
            parsed.map
        );
        let expected_start: Point = (0, 4).into();
        let expected_end: Point = (5, 2).into();
        assert_eq!(expected_start, parsed.start);
        assert_eq!(expected_end, parsed.end);

        assert_eq!(
            'z' as i64 - 'a' as i64,
            parsed.height_at(&expected_end)
        )
    }

    #[test]
    fn test_example_shorted_path() {
        let input = include_str!("input.example.txt");
        let parsed: HeightMap = input.parse().unwrap();
        let start = &parsed.start.clone();
        let end = &parsed.end.clone();
        let plane = parsed.to_plane();

        let shortest_path = astar::astar(
            start.clone(), 
            end.clone(), 
            &StraightLine, 
            &OnlyUpOneCost(&parsed, StraightLine), 
            &DirectNeighbours(&plane),
        ).unwrap();
        
        assert_eq!(31, shortest_path.path.len());
    }

}