use std::{collections::HashSet, time::Instant, str::FromStr, i64};

use aoclib::{astar::{Cost, StraightLine}, cartesian::{Point, Plane}, neighbour::DirectNeighbours};
use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let g: Grid = txt.parse().expect("valid grid");
    let mut start_pos = None;    


    let height = g.height();
    let width = g.width();

    for row in 0..height {
        let y = row;
        for x in 0..width {
            if g.0[y][x] == Tile::Start {
                start_pos = Some(Point{ x: x as i64, y: y as i64});
                break;
            }
        }
    }
    let start_pos = start_pos.expect("must be a start");
    println!("start: {:?}", start_pos);
    let mut max = 0;

    let plane = g.clone().into();

    for y in 0..height {
        for x in 0..width {
            let point = Point{ x: x as i64, y: y as i64};
            let result = aoclib::astar::astar(
                point.clone(), 
                start_pos.clone(), 
                &StraightLine, 
                &g, 
                &DirectNeighbours(&plane), 
            );
            max = max.max(result.map(|p| p.path.len()).unwrap_or(0));
        }
    }

    max
} 

fn part2(_txt: &str) -> i64 {
    1
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    // | is a vertical pipe connecting north and south.
    Vertical,    

    // - is a horizontal pipe connecting east and west.
    Horizonal,

    // L is a 90-degree bend connecting north and east.
    NorthAndEastBend,

    // J is a 90-degree bend connecting north and west.
    NorthAndWestBend,

    // 7 is a 90-degree bend connecting south and west.
    SouthAndWestBend,

    // F is a 90-degree bend connecting south and east.
    SouthAndEastBend,

    // . is ground; there is no pipe in this tile.
    Ground,

    // S is the starting position of the animal; there is a pipe on this
    Start,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Connects {
    Left,
    Right,
    Up,
    Down,
}

impl Tile {
    fn connects(&self) -> HashSet<Connects> {
        match self {
            Tile::Start => HashSet::from_iter(vec![Connects::Left, Connects::Right, Connects::Up, Connects::Down]),
            Tile::Ground => HashSet::new(),
            Tile::NorthAndEastBend => HashSet::from_iter(vec![Connects::Up, Connects::Right]),
            Tile::NorthAndWestBend => HashSet::from_iter(vec![Connects::Up, Connects::Left]),
            Tile::SouthAndEastBend => HashSet::from_iter(vec![Connects::Down, Connects::Right]),
            Tile::SouthAndWestBend => HashSet::from_iter(vec![Connects::Down, Connects::Left]),
            Tile::Vertical => HashSet::from_iter(vec![Connects::Up, Connects::Down]),
            Tile::Horizonal => HashSet::from_iter(vec![Connects::Left, Connects::Right]),
        }
    }
}


#[derive(Debug, Clone)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

impl From<Grid> for Plane {
    fn from(value: Grid) -> Self {
        let max_y = (value.0.len() - 1) as i64;
        let max_x = (value.0[0].len() - 1) as i64;
        Plane { 
            top_left: (0, max_y).into(), 
            bottom_right: Point { x: max_x, y: 0 }
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = s.lines().map(|l| l.chars().map(|c| {
            match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizonal,
                'L' => Tile::NorthAndEastBend,
                'J' => Tile::NorthAndWestBend,
                '7' => Tile::SouthAndWestBend,
                'F' => Tile::SouthAndEastBend,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => unreachable!("bad tile"),
            }
        }).collect_vec())
        .collect_vec();
        grid.reverse();
        Ok(Self(grid))
    }
}

impl Cost for Grid {
    fn measure(&self, from: &aoclib::cartesian::Point, to: &aoclib::cartesian::Point) -> i64 {
        let from_tile = self.0[from.y as usize][from.x as usize];
        let to_tile = self.0[to.y as usize][to.x as usize];

        let impossible = 1000000000000000;

        let x_delta = to.x - from.x;
        let y_delta = to.y - from.y;

        match (x_delta, y_delta) {
            (1, 0) => {
                // going right
                if to_tile.connects().contains(&Connects::Left) && from_tile.connects().contains(&Connects::Right) {
                    1
                } else {
                    impossible
                }
            },
            (-1, 0) => {
                // going left
                if to_tile.connects().contains(&Connects::Right) && from_tile.connects().contains(&Connects::Left) {
                    1
                } else {
                    impossible
                }
            },
            (0, 1) => {
                // going up
                if to_tile.connects().contains(&Connects::Down) && from_tile.connects().contains(&Connects::Up) {
                    1
                } else {
                    impossible
                }
            },
            (0, -1) => {
                // going down
                if to_tile.connects().contains(&Connects::Up) && from_tile.connects().contains(&Connects::Down) {
                    1
                } else {
                    impossible
                }
            },
            _ => impossible
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(4, part1(include_str!("input.test.txt")));
    }


    #[test]
    fn test_example_p2() {
        assert_eq!(1, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse_grid() {
        let input = include_str!("input.test.txt");
        let g = input.parse::<Grid>();

        assert_eq!(true, g.is_ok());
    }

    #[test]
    fn test_grid_connections() {
        let input = include_str!("input.test.txt");
        let g = input.parse::<Grid>().unwrap();

        let p_one = (0, 0).into();
        let p_two = (1, 0).into();
        
        let cost = g.measure(&p_one, &p_two);
        assert_eq!(true, cost > 1);
        
        let p_one = (1, 1).into();
        let p_two = (2, 1).into();
        assert_eq!(1, g.measure(&p_one, &p_two));
    }
    
}
