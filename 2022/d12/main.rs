use std::str::FromStr;

use aoclib::{
    grid::{FromChar, Grid, GridPosition},
    shortest_path::*,
    shortest_path::{self, Cost, NonDiagonalNeighbours},
};

fn main() {
    let input = include_str!("input.txt");
    let pt1 = part1(input);
    println!("pt1: {}", pt1);
    let pt2 = part2(input);
    println!("pt2: {}", pt2);
}

fn part1(input: &str) -> usize {
    let parsed: HeightMap = input.parse().unwrap();
    let start = &parsed.start;
    let end = &parsed.end;
    let heuristic = ManhattenDistanceTo(*end);

    let shortest_path = shortest_path::astar(
        &parsed,
        &parsed,
        &heuristic,
        start.clone(),
        |p: &GridPosition| p == end,
    )
    .unwrap();
    shortest_path.path.len()
}

fn part2(input: &str) -> i64 {
    let parsed: HeightMap = input.parse().unwrap();
    let end = &parsed.end;
    let heuristic = ManhattenDistanceTo(*end);

    parsed
        .map
        .rows
        .iter()
        .enumerate()
        .flat_map(|(row_num, row)| {
            row.iter().enumerate().filter_map(move |(col_num, h)| {
                if h.height() == 0 {
                    Some(GridPosition::new(col_num, row_num))
                } else {
                    None
                }
            })
        })
        .filter_map(|start| {
            let maybe_p =
                shortest_path::astar(&parsed, &parsed, &heuristic, start, |p: &GridPosition| {
                    p == end
                });
            maybe_p.map(|p| p.path.len() as i64)
        })
        .min()
        .unwrap_or(-1)
}

#[derive(Debug)]
enum MapPoint {
    Start,
    End,
    Height(char),
}

impl FromChar for MapPoint {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c {
            'S' => Ok(MapPoint::Start),
            'E' => Ok(MapPoint::End),
            other => Ok(MapPoint::Height(other)),
        }
    }
}

impl MapPoint {
    fn height(&self) -> i64 {
        match self {
            MapPoint::Start => 0,
            MapPoint::End => ('z' as i64) - ('a' as i64),
            MapPoint::Height(c) => (*c as i64) - ('a' as i64),
        }
    }
}

#[derive(Debug)]
struct HeightMap {
    map: Grid<MapPoint>,
    start: GridPosition,
    end: GridPosition,
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Grid<MapPoint> = s.parse().unwrap();

        let mut start: Option<GridPosition> = None;
        let mut end: Option<GridPosition> = None;

        for (row_num, row) in map.rows.iter().enumerate() {
            for (col_num, col) in row.iter().enumerate() {
                match col {
                    MapPoint::Start => {
                        start = Some(GridPosition::new(col_num, row_num));
                    }
                    MapPoint::End => {
                        end = Some(GridPosition::new(col_num, row_num));
                    }
                    MapPoint::Height(_) => {}
                }
            }
        }

        Ok(HeightMap {
            start: start.ok_or_else(|| "start missing")?,
            end: end.ok_or_else(|| "end mising".to_string())?,
            map,
        })
    }
}

impl Cost<GridPosition, i64> for HeightMap {
    fn measure(&self, from: &GridPosition, to: &GridPosition) -> i64 {
        let to_height = self.map.at(&to).height();
        let from_height = self.map.at(&from).height();
        if (to_height - from_height) > 1 {
            i64::impossible()
        } else {
            1
        }
    }
}

impl Neighbours<GridPosition> for HeightMap {
    fn neighbours(&self, state: &GridPosition) -> Vec<GridPosition> {
        NonDiagonalNeighbours(&self.map).neighbours(&state)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("input.example.txt");
        let parsed: HeightMap = input.parse().unwrap();
        let expected_start = GridPosition::new(0, 0);
        let expected_end = GridPosition::new(5, 2);
        assert_eq!(expected_start, parsed.start);
        assert_eq!(expected_end, parsed.end);
        let heights = parsed
            .map
            .rows
            .iter()
            .rev()
            .map(|row| row.iter().map(|col| col.height()).collect_vec())
            .collect_vec();
        assert_eq!(
            vec![
                vec![0, 1, 3, 4, 5, 6, 7, 8],
                vec![0, 2, 2, 19, 20, 21, 22, 9],
                vec![0, 2, 2, 18, 25, 25, 23, 10],
                vec![0, 1, 2, 17, 24, 23, 23, 11],
                vec![0, 0, 1, 16, 15, 14, 13, 12]
            ],
            heights
        );
    }

    #[test]
    fn test_cost() {
        let input = include_str!("input.example.txt");
        let parsed: HeightMap = input.parse().unwrap();
        let cost = parsed.measure(&GridPosition::new(0, 0), &GridPosition::new(1, 0));
        assert_eq!(1, cost);
        let cost = parsed.measure(&GridPosition::new(2, 0), &GridPosition::new(3, 0));
        assert_eq!(i64::impossible(), cost);
    }

    #[test]
    fn test_example_shorted_path() {
        let input = include_str!("input.example.txt");
        let parsed: HeightMap = input.parse().unwrap();
        let start = &parsed.start.clone();
        let end = &parsed.end.clone();
        let heuristic = ManhattenDistanceTo(*end);

        let end_check = |p: &GridPosition| {
            // println!("checking: {:?} to be {:?}", m.to, end);
            p == end
        };

        let shortest_path =
            shortest_path::astar(&parsed, &parsed, &heuristic, start.clone(), end_check).unwrap();

        assert_eq!(31, shortest_path.path.len());
    }

    #[test]
    fn test_pt2_example() {
        let input = include_str!("input.example.txt");
        let result = part2(input);

        assert_eq!(29, result);
    }
}
