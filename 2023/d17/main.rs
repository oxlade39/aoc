use core::fmt;
use std::{
    collections::{HashSet, HashMap},
    fmt::{Debug, Display},
    str::FromStr,
    time::Instant, iter,
};

use aoclib::{
    astar::{Cost, StraightLine, Multiplier, NeighbourState},
    cartesian::{Plane, Point},
    input::{Flip, FromChar, Grid},
    neighbour::{DirectNeighbours, Neighbours},
};

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let lf: LavaFall = txt.parse().unwrap();
    let plane: Plane = (&lf.map).into();

    let start: Point = (0, (lf.map.height() - 1) as i64).into();
    let end = ((lf.map.width() - 1) as i64, 0).into();
    let heuristic = Multiplier(StraightLine, 2);
    let dn = DirectNeighbours(&plane);
    let neighbours = No3InARow(&dn);
    let path =
        aoclib::astar::astar(start.clone(), end, &heuristic, &lf, &neighbours).unwrap();

    for row in 0..lf.map.height() {
        let y = lf.map.height() - row - 1;
        for x in 0..lf.map.width() {
            let current: Point = (x as i64, y as i64).into();
            if current == start || path.path.iter().any(|(p, _)| {                
                &current == p
            }) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }

    // for (p, cost) in path.path {
    //     println!("{p:?} = {cost}");
    // }

    path.total_cost as usize
}

fn part2(txt: &str) -> usize {
    0
}

struct LavaFall {
    map: Grid<u32>,
}

struct No3InARow<'a, 'b>(&'b DirectNeighbours<'a>);

impl<'ns, 'n, 'b> Neighbours<NeighbourState<'ns>> for No3InARow<'n, 'b> {
    fn neighbours(&self, ns: &NeighbourState<'ns>) -> Vec<Point> {
        let _delegate = self.0;
        let p = ns.current_point;
        if let Some(prev) = ns.came_from.get(p) {

            let mut lookback: HashSet<(i64, i64)> = HashSet::new();
            let mut count = 0;
            let mut current = p;
            while let Some(n) = ns.came_from.get(current) {

                let x_diff = (current.x - n.x).signum();
                let y_diff = (current.y - n.y).signum();  

                lookback.insert((x_diff, y_diff));
                count += 1;
                current = n;

                if count == 4 {
                    break;
                }
            }

            let x_diff = (p.x - prev.x).signum();
            let y_diff = (p.y - prev.y).signum();
            
            let results = if lookback.len() == 1 && count == 4 {
                // all same direction for last 3
                // so remove same dir next
                match (x_diff, y_diff) {
                    // going up
                    (0, 1) => vec![
                        p.transform(&(-1, 0).into()),
                        p.transform(&(1, 0).into()),
                    ],
                    // going down
                    (0, -1) => vec![
                        p.transform(&(-1, 0).into()),
                        p.transform(&(1, 0).into()),
                    ],
                    // going left
                    (-1, 0) => vec![
                        p.transform(&(0, 1).into()),
                        p.transform(&(0, -1).into()),
                    ],
                    // going right
                    (1, 0) => vec![
                        p.transform(&(0, 1).into()),
                        p.transform(&(0, -1).into()),
                    ],
                    _ => panic!("bad point"),
                }
            } else {
                match (x_diff, y_diff) {
                    // going up
                    (0, 1) => vec![
                        p.transform(&(0, 1).into()),
                        p.transform(&(-1, 0).into()),
                        p.transform(&(1, 0).into()),
                    ],
                    // going down
                    (0, -1) => vec![
                        p.transform(&(0, -1).into()),
                        p.transform(&(-1, 0).into()),
                        p.transform(&(1, 0).into()),
                    ],
                    // going left
                    (-1, 0) => vec![
                        p.transform(&(-1, 0).into()),
                        p.transform(&(0, 1).into()),
                        p.transform(&(0, -1).into()),
                    ],
                    // going right
                    (1, 0) => vec![
                        p.transform(&(1, 0).into()),
                        p.transform(&(0, 1).into()),
                        p.transform(&(0, -1).into()),
                    ],
                    _ => panic!("bad point"),
                }
            };

            return results.into_iter().filter(|p| p.within(self.0.0)).collect();

        }

        vec![p.transform(&(1, 0).into())]
        // panic!("never here");
        // delegate.neighbours(ns)
        //     .into_iter()
        //     .filter(|p| !ns.came_from.contains_key(p))
        //     .collect()

    }
}

impl FromStr for LavaFall {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Grid<u32>>() {
            Ok(g) => Ok(LavaFall { map: g.flip() }),
            Err(e) => Err(e),
        }
    }
}

const IMPOSSIBLE: i64 = 10000000000;

impl Cost for LavaFall {
    fn measure(
        &self,
        from: &Point,
        to: &Point,
    ) -> i64 {        
        self.map.rows[to.y as usize][to.x as usize].into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(102, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(0, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse() {
        let txt = include_str!("input.test.txt");
        let lf = txt.parse::<LavaFall>().unwrap();

        assert_eq!(2, lf.map.rows[lf.map.height() - 1][0]);
        assert_eq!(4, lf.map.rows[lf.map.height() - 1][1]);
        assert_eq!(1, lf.map.rows[lf.map.height() - 1][2]);

        assert_eq!(3, lf.map.rows[lf.map.height() - 2][0]);
        assert_eq!(2, lf.map.rows[lf.map.height() - 2][1]);
        assert_eq!(1, lf.map.rows[lf.map.height() - 2][2]);
    }

    #[test]
    fn test_cost() {
        let txt = include_str!("input.test.txt");
        let lf = txt.parse::<LavaFall>().unwrap();

        let from: Point = (0 as i64, (lf.map.height() - 1) as i64).into();
        let to: Point = (0 as i64, (lf.map.height() - 2) as i64).into();

        let cost = lf.measure(&from, &to);
        assert_eq!(3, cost);
    }

    #[test]
    fn test_smaller_example() {
        let txt = "\
        9333\n\
        9393\n\
        0110\n\
        9229";
        let lf: LavaFall = txt.parse().unwrap();
        
        let plane: Plane = (&lf.map).into();

        let start: Point = (0, 1).into();
        let end = (3, 1).into();
        let heuristic = Multiplier(StraightLine, 2);
        let dn = DirectNeighbours(&plane);
        let neighbours = No3InARow(&dn);
        let path =
            aoclib::astar::astar(start.clone(), end, &heuristic, &lf, &neighbours).unwrap();

        for row in 0..lf.map.height() {
            let y = lf.map.height() - row - 1;
            for x in 0..lf.map.width() {
                let current: Point = (x as i64, y as i64).into();
                if current == start || path.path.iter().any(|(p, _)| {                
                    &current == p
                }) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
    }
}
