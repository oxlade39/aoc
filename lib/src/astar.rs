use std::collections::{BTreeSet, HashMap};

use crate::{cartesian::{Point, Plane, Vector, Transform}, distance::StraightLineDistance};

/// Use sufficiently high number that a real hueristic wouldn't be above
const INFINITY: i64 = 1000000;

#[derive(Clone, Debug)]
struct Candidate { 
    point: Point,
    cost: i64
}

impl Candidate {
    fn new(point: Point, cost: i64) -> Self {
        Candidate { point, cost }
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.point.eq(&other.point)
    }
}

impl Eq for Candidate {}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.point.cmp(&other.point)
    }
}

trait Hueristic {
    /// must underestimate the actual cost for a* to find shortest path
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

trait Cost {
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

trait Neighbours {
    fn neighbours(&self, p: &Point) -> Vec<Point>;
}

struct StraightLine;

impl Hueristic for StraightLine {
    fn measure(&self, from: &Point, to: &Point) -> i64 {
        let v: Vector = (from.clone(), to.clone()).into();
        let sld: StraightLineDistance = v.into();
        sld.0
    }
}

impl Cost for StraightLine {
    fn measure(&self, from: &Point, to: &Point) -> i64 {
        let v: Vector = (from.clone(), to.clone()).into();
        let sld: StraightLineDistance = v.into();
        sld.0
    }
}

impl Cost for Vec<Vec<i64>> {
    fn measure(&self, _: &Point, to: &Point) -> i64 {
        self[to.y as usize][to.x as usize]
    }
}

struct DirectNeighbours<'a>(&'a Plane);
struct TouchingNeighbours<'a>(&'a Plane);

impl Neighbours for DirectNeighbours<'_> {
    fn neighbours(&self, p: &Point) -> Vec<Point> {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .map(|t| {
                let into: Transform = t.into();
                into
            })
            .into_iter()
            .filter_map(|t| {
                let transformed = p.transform(&t);
                if transformed.within(self.0) {
                    Some(transformed)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Neighbours for TouchingNeighbours<'_> {
    fn neighbours(&self, p: &Point) -> Vec<Point> {
        [(-1, 0), (1, 0), (0, 1), (0, -1), (-1, -1), (1, 1), (-1, 1), (1, -1)]
            .map(|t| {
                let into: Transform = t.into();
                into
            })
            .into_iter()
            .filter_map(|t| {
                let transformed = p.transform(&t);
                if transformed.within(self.0) {
                    Some(transformed)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Debug, PartialEq)]
struct ShortestPath {
    path: Vec<(Point, i64)>,
    total_cost: i64
}

fn astar<H, C, N>(
    start: Point, 
    end: Point,
    heuristic: H,
    cost: C,
    neighbours: N
) -> ShortestPath 
where H: Hueristic, C: Cost, N: Neighbours
{
    let mut open_set: BTreeSet<Candidate> = BTreeSet::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut g_scores: HashMap<Point, i64> = HashMap::new();
    let mut f_scores: HashMap<Point, i64> = HashMap::new();


    let start_f_score = heuristic.measure(&start, &end);
    open_set.insert(Candidate::new(start.clone(), start_f_score));
    g_scores.insert(start.clone(), 0);
    f_scores.insert(start.clone(), start_f_score);

    loop {
        if open_set.is_empty() {
            break;
        }

        // this is slow, optimise to use a priority queue
        let curr_candid = open_set.first().unwrap().clone();
        let curr_node = curr_candid.point.clone();

        if curr_node == end {
            let mut path: Vec<(Point, i64)> = vec![];
            let mut path_node = Some(curr_node);
            let mut total_cost = 0;

            while let Some(p) = path_node  {
                let next = came_from.remove(&p);
                if let Some(ref p1) = next {
                    let node_cost = cost.measure(&p1, &p);
                    path.push((p, node_cost));
                    total_cost += node_cost;
                }
                path_node = next;
            }
            // fix to take ownership of points
            return ShortestPath {
                path,
                total_cost,
            };
        }

        open_set.remove(&curr_candid);
        for neighbour in neighbours.neighbours(&curr_node) {
            let neighbour_cost = cost.measure(&curr_node, &neighbour);
            let neighbour_g_score = g_scores.get(&neighbour).unwrap_or(&INFINITY);
            let tentative_g_score = g_scores.get(&curr_node).unwrap_or(&INFINITY) + neighbour_cost;
            if tentative_g_score < *neighbour_g_score {
                came_from.insert(neighbour.clone(), curr_node.clone());
                g_scores.insert(neighbour.clone(), tentative_g_score);

                // distance to target
                let hueristic = heuristic.measure(&neighbour, &end);
                f_scores.insert(neighbour.clone(), tentative_g_score + hueristic);
                open_set.insert(Candidate::new(neighbour.clone(), tentative_g_score + hueristic));
            }
        }
    }

    ShortestPath { path: vec![], total_cost: -1 }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;

    struct ManhattenDistance;

    impl Hueristic for ManhattenDistance {
        fn measure(&self, from: &Point, to: &Point) -> i64 {
            (from.x - to.x).abs() + (from.y - to.y).abs()
        }
    }

    impl Cost for ManhattenDistance {
        fn measure(&self, from: &Point, to: &Point) -> i64 {
            (from.x - to.x).abs() + (from.y - to.y).abs()
        }
    }

    #[test]
    fn test_direct_neighbours_at_edge() {
        let p: Point = (0, 0).into();
        let plane: Plane = (10, 10).into();
        let neighbours = DirectNeighbours(&plane);
        let n = neighbours.neighbours(&p);

        let expected: Vec<Point> = vec![            
            (1, 0).into(),
            (0, 1).into(),
        ];
        assert_eq!(n, expected);
    }

    #[test]
    fn test_touching_neighbours_includes_diagonals() {
        // ...
        // .P.
        // ...
        let plane: Plane = (3, 3).into();
        let p: Point = (1, 1).into();

        let expected: HashSet<Point> = HashSet::from_iter(vec![
            (0, 2), (1, 2), (2, 2),
            (0, 1),         (2, 1),
            (0, 0), (1, 0), (2, 0)
        ].into_iter()
        .map(|p| p.into()));

        let neighbours = TouchingNeighbours(&plane);
        let n: HashSet<Point> = HashSet::from_iter(neighbours.neighbours(&p));

        assert_eq!(n, expected);
    }

    #[test]
    fn test_example() {
        let plane = &(10, 10).into();
        let start = (0, 0).into();
        let end = (5, 5).into();

        let result = astar(
            start, 
            end, 
            ManhattenDistance, 
            ManhattenDistance,
            DirectNeighbours(&plane),
        );

        assert_eq!(10, result.total_cost);
    }

    #[test]
    fn test_simple_straight_line_example() {
        // .E
        // S.

        // .E
        // S.

        let plane = (2, 2).into();
        let start = (0, 0).into();
        let end = (1, 1).into();
        let heuristic = StraightLine;
        let cost = StraightLine;
        
        let shortest_path = astar(
            start, 
            end, 
            heuristic, 
            cost,
            TouchingNeighbours(&plane),
        );
        assert_eq!(1, shortest_path.total_cost, "{:?}", shortest_path);
    }

    #[test]
    fn test_straight_line_example() {
        // ....E
        // .....
        // .....
        // S....

        // ...-E
        // ../..
        // ./...
        // S....

        let plane = (5, 4).into();
        let start = (0, 0).into();
        let end = (4, 3).into();
        let heuristic = StraightLine;
        let cost = StraightLine;
        
        let shortest_path = astar(
            start, 
            end, 
            heuristic, 
            cost,
            TouchingNeighbours(&plane),
        );
        assert_eq!(4, shortest_path.total_cost, "{:?}", shortest_path);
    }

    #[test]
    fn test_grid_cost_example() {
        // S....
        // ####.
        // .....
        // .####
        // ....E

        // S--\
        // ####|
        // ./-/.
        // |####
        // .\--E
        let plane = (5, 5).into();
        let start = (0, 4).into();
        let end = (4, 0).into();
        let heuristic = StraightLine;

        // construct back to front so indices line up
        let cost = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, INFINITY, INFINITY, INFINITY, INFINITY],
            vec![1, 1, 1, 1, 1],
            vec![INFINITY, INFINITY, INFINITY, INFINITY, 1],
            vec![1, 1, 1, 1, 1],
        ];
        
        let shortest_path = astar(
            start, 
            end, 
            heuristic, 
            cost,
            TouchingNeighbours(&plane),
        );
        assert_eq!(12, shortest_path.total_cost, "{:?}", shortest_path);
    }

}