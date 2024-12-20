use core::str;
use std::{
    fmt::Display, str::FromStr, time::Instant, usize
};

use aoclib::{
    grid::{FromChar, Grid, GridPosition},
    input, timing,
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}", timing::format_elapsed_time(now.elapsed()));
}

fn part1(txt: &str) -> usize {
    let mut puzzle: Puzzle = txt.parse().unwrap();
    puzzle.apply_moves();
    sum_of_gps(&Tile::Box, &puzzle.map)
}

fn part2(txt: &str) -> usize {
    let mut puzzle: Puzzle2 = txt.parse::<Puzzle>().unwrap().into();
    puzzle.apply_moves();
    sum_of_gps(&Tile2::LeftBox, &puzzle.map)
}

#[derive(Debug)]
struct Puzzle {
    map: Grid<Tile>,
    directions: Vec<Direction>,
}

fn find_one<T>(
    needle: T, 
    haystack: &Grid<T>
) -> GridPosition 
where T: Eq,
    T: Display
{
    for row in 0..haystack.height() {
        for col in 0..haystack.width() {
            let p = GridPosition::new(col, row);
            if *haystack.at(&p) == needle {
                return p;
            }
        }
    }
    panic!("{needle} not found")
}

fn sum_of_gps<T>(
    box_type: &T,
    grid: &Grid<T>,
) -> usize 
where T: Eq,
{
    let mut sum = 0;
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let p = GridPosition::new(col, row);
            if grid.at(&p) == box_type {
                sum += 100 * row + col;
            }
        }
    }
    sum
}

impl Puzzle {
    fn apply_moves(&mut self) {
        let mut robot = find_one(Tile::Robot, &self.map);
        for i in 0..self.directions.len() {            
            let d = self.directions[i].clone();
            // println!("Move {}:", d);
            let moved = self.apply_move(&robot, &d);
            if let Some(m) = moved {
                robot = m;
            }            
            // println!("{}", self.map);
            // println!("robot at {:?}\n\n", robot);
        }
    }

    fn apply_move(
        &mut self, 
        p: &GridPosition, 
        d: &Direction
    ) -> Option<GridPosition> {
        let next_pos = d.apply(p);
        let next_tile = self.map.at(&next_pos);

        // println!("next tile after {} is {}", d, next_tile);

        let was_moved = match next_tile {
            Tile::Wall | Tile::Robot => return None,
            Tile::Box => {
                self.apply_move(&next_pos, d)
            },
            Tile::Space => {
                // can move
                Some(next_pos)

            },
        };
        if let Some(_moved) = was_moved {
            // move current into freed space
            let tile_at_p = self.map.at(&p);
            self.map.rows[next_pos.row][next_pos.col] = tile_at_p.clone();
            self.map.rows[p.row][p.col] = Tile::Space;
            Some(next_pos)
        } else {
            None
        }
    }
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = input::empty_line_chunks(s);
        let top = parts.next().unwrap();
        let bottom = parts.next().unwrap();

        let map = top.parse().unwrap();
        let directions: Vec<Direction> = bottom
            .lines()
            .flat_map(|line| line.chars().map(|c| Direction::from_char(c).unwrap()))
            .collect();

        Ok(Puzzle { map, directions })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Box,
    Space,
    Robot,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile2 {
    Wall,
    LeftBox,
    RightBox,
    Space,
    Robot,
}

impl FromChar for Tile {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Space),
            'O' => Ok(Tile::Box),
            '@' => Ok(Tile::Robot),
            other => Err(format!("bad tile '{}'", other)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Box => f.write_str("O"),
            Tile::Robot => f.write_str("@"),
            Tile::Space => f.write_str("."),
            Tile::Wall => f.write_str("#"),
        }
    }
}

impl Display for Tile2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile2::LeftBox => f.write_str("["),
            Tile2::RightBox => f.write_str("]"),
            Tile2::Robot => f.write_str("@"),
            Tile2::Space => f.write_str("."),
            Tile2::Wall => f.write_str("#"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn apply(&self, p: &GridPosition) -> GridPosition {
        match self {
            Direction::Left => p.left(),
            Direction::Right => p.right(),
            Direction::Up => p.up(),
            Direction::Down => p.down(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => f.write_str("<"),
            Direction::Right => f.write_str(">"),
            Direction::Up => f.write_str("^"),
            Direction::Down => f.write_str("v"),
        }
    }
}

impl FromChar for Direction {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            other => Err(format!("bad direction '{}'", other)),
        }
    }
}

struct Puzzle2 {
    map: Grid<Tile2>,
    directions: Vec<Direction>,
}

impl From<Puzzle> for Puzzle2 {
    fn from(value: Puzzle) -> Self {
        let rows = value.map.rows.into_iter()
            .map(|row| row.into_iter().flat_map(|col| match col {
                Tile::Wall => vec![Tile2::Wall, Tile2::Wall].into_iter(),
                Tile::Box => vec![Tile2::LeftBox, Tile2::RightBox].into_iter(),
                Tile::Space => vec![Tile2::Space, Tile2::Space].into_iter(),
                Tile::Robot => vec![Tile2::Robot, Tile2::Space].into_iter(),
            }).collect())
            .collect();

        Puzzle2 { map: Grid { rows }, directions: value.directions }
    }
}

impl Puzzle2 {
    fn apply_moves(&mut self) {
        let mut robot = find_one(Tile2::Robot, &self.map);
        for i in 0..self.directions.len() {            
            let d = &self.directions[i].clone();
            // println!("Move {}:", d);

            match d {
                Direction::Left | Direction::Right => {
                    let moved = self.apply_horizontal_move(&robot, d);
                    if let Some(moved) = moved {
                        robot = moved;
                    }            
                    // println!("{}", self.map);
                    // println!("robot at {:?}\n\n", robot);                
                },
                Direction::Up | Direction::Down => {
                    if self.can_move_vertically(robot, d.clone()) {
                        self.move_vertically(robot, d.clone());
                        robot = d.apply(&robot);
                        // println!("{}", self.map);
                        // println!("robot at {:?}\n\n", robot);                
                    }                    
                },
            }            
        }
    }

    fn apply_horizontal_move(
        &mut self, 
        p: &GridPosition, 
        d: &Direction
    ) -> Option<GridPosition> {
        let next_pos = d.apply(p);
        let next_tile = self.map.at(&next_pos);

        // println!("next tile after {} is {}", d, next_tile);

        let was_moved = match next_tile {
            Tile2::Wall | Tile2::Robot => return None,
            Tile2::LeftBox | Tile2::RightBox => {
                self.apply_horizontal_move(&next_pos, d)
            },
            Tile2::Space => {
                // can move
                Some(next_pos)

            },
        };
        if let Some(_moved) = was_moved {
            // move current into freed space
            let tile_at_p = self.map.at(&p);
            self.map.rows[next_pos.row][next_pos.col] = tile_at_p.clone();
            self.map.rows[p.row][p.col] = Tile2::Space;
            Some(next_pos)
        } else {
            None
        }
    }

    fn can_move_vertically(&self, p: GridPosition, d: Direction) -> bool {
        let next_pos = d.apply(&p);
        let next_tile = self.map.at(&next_pos);

        match next_tile {
            Tile2::Wall => false,
            Tile2::Space => true,
            Tile2::Robot => true,
            Tile2::LeftBox => {
                self.can_move_vertically(next_pos, d.clone()) && self.can_move_vertically(next_pos.right(), d.clone())
            },
            Tile2::RightBox => {
                self.can_move_vertically(next_pos, d.clone()) && self.can_move_vertically(next_pos.left(), d.clone())
            },                        
        }
    }

    fn move_vertically(&mut self, p: GridPosition, d: Direction) {
        let next_pos = d.apply(&p);
        let next_tile = self.map.at(&next_pos);

        match next_tile {            
            Tile2::LeftBox => {
                self.move_vertically(next_pos, d.clone());
                self.move_vertically(next_pos.right(), d.clone());
            },
            Tile2::RightBox => {
                self.move_vertically(next_pos.left(), d.clone());
                self.move_vertically(next_pos, d.clone()); 
            },
            Tile2::Space | Tile2::Robot => {
                // let tile_at_p = self.map.at(&p);
                // self.map.rows[next_pos.row][next_pos.col] = tile_at_p.clone();
                // self.map.rows[p.row][p.col] = Tile2::Space;
            },
            Tile2::Wall => panic!("next tile is wall"),
        }
        let tile_at_p = self.map.at(&p);
        self.map.rows[next_pos.row][next_pos.col] = tile_at_p.clone();
        self.map.rows[p.row][p.col] = Tile2::Space;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_input_pt1_parse() {
        let test_input = include_str!("input.test.small.txt");
        let mut puzzle: Puzzle = test_input.parse().unwrap();
        // println!("{}", puzzle.map);
        puzzle.apply_moves();
        assert_eq!(2028, sum_of_gps(&Tile::Box, &puzzle.map));
    }

    #[test]
    fn test_gps_sum() {
        let test_input = include_str!("input.sum.txt");
        let puzzle: Puzzle = test_input.parse().unwrap();
        // println!("{}", puzzle.map);
        assert_eq!(104, sum_of_gps(&Tile::Box, &puzzle.map));
    }

    #[test]
    fn test_input_pt1() {
        let test_input = include_str!("input.test.txt");
        let mut puzzle: Puzzle = test_input.parse().unwrap();
        // println!("{}", puzzle.map);
        puzzle.apply_moves();

        // println!("Final:\n{}", puzzle.map);
        assert_eq!(10092, sum_of_gps(&Tile::Box, &puzzle.map));
    }

    #[test]
    fn input_pt1() {
        let test_input = include_str!("input.txt");
        assert_eq!(1515788, part1(test_input));
    }

    #[test]
    fn test_input_puzzle2_into() {
        let test_input = include_str!("input.test.txt");
        let puzzle: Puzzle = test_input.parse().unwrap();
        let p2: Puzzle2 = puzzle.into();
        // println!("{}", p2.map);
        assert_eq!(20, p2.map.width());
    }

    #[test]
    fn test_input_pt2_steps() {
        let test_input = include_str!("input.test.txt");
        let mut puzzle: Puzzle2 = test_input.parse::<Puzzle>().unwrap().into();
        // println!("{}", puzzle.map);
        puzzle.apply_moves();
        // println!("Final Map:\n{}", puzzle.map);
        assert_eq!(9021, sum_of_gps(&Tile2::LeftBox, &puzzle.map));
    }

    #[test]
    fn input_pt2() {
        let test_input = include_str!("input.txt");
        assert_eq!(1516544, part2(test_input));
    }
}
