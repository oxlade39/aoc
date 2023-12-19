use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::{
    cartesian::{Point, Vector},
    distance::StraightLineDistance,
    neighbour::Neighbours,
};

/// Use sufficiently high number that a real hueristic wouldn't be above
const INFINITY: i64 = 1000000;

#[derive(Clone, Debug)]
struct Candidate {
    point: Point,
    cost: i64,
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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An heuristic function used to estimate the h-score
/// See the <a href="https://en.wikipedia.org/wiki/A*_search_algorithm#Description">description here</a>
pub trait Hueristic {
    /// must underestimate the actual cost for a* to find shortest path
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

/// A cost function used to estimate the path cost
/// See the <a href="https://en.wikipedia.org/wiki/A*_search_algorithm#Description">description here</a>
pub trait Cost {
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

pub struct StraightLine;

impl Hueristic for StraightLine {
    fn measure(&self, from: &Point, to: &Point) -> i64 {
        let v: Vector = (from.clone(), to.clone()).into();
        let sld: StraightLineDistance = v.into();
        sld.0
    }
}

pub struct Multiplier<T: Hueristic>(pub T, pub i64);

impl<T: Hueristic> Hueristic for Multiplier<T> {
    fn measure(&self, from: &Point, to: &Point) -> i64 {
        self.1 * self.0.measure(from, to)
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

#[derive(Debug, PartialEq)]
pub struct ShortestPath {
    pub path: Vec<(Point, i64)>,
    pub total_cost: i64,
}

pub struct NeighbourState<'a> {
    pub current_point: &'a Point,
    pub came_from: &'a HashMap<Point, Point>,
}

/// Extendable implementation of
/// <a href="https://en.wikipedia.org/wiki/A*_search_algorithm">astar</a> shortest path.
pub fn astar<H, C, N>(
    start: Point,
    end: Point,
    heuristic: &H,
    cost: &C,
    neighbours: &N,
) -> Option<ShortestPath>
where
    H: Hueristic,
    C: Cost,
    N: for<'a> Neighbours<NeighbourState<'a>>,
{
    let mut open_set: BinaryHeap<Candidate> = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut g_scores: HashMap<Point, i64> = HashMap::new();
    let mut f_scores: HashMap<Point, i64> = HashMap::new();

    let start_f_score = heuristic.measure(&start, &end);
    open_set.push(Candidate::new(start.clone(), start_f_score));
    g_scores.insert(start.clone(), 0);
    f_scores.insert(start.clone(), start_f_score);

    while let Some(curr_candid) = open_set.pop() {
        let curr_node = curr_candid.point;

        if curr_node == end {
            let mut path: Vec<(Point, i64)> = vec![];
            let mut path_node = Some(curr_node);
            let mut total_cost = 0;

            while let Some(p) = path_node {
                let next = came_from.remove(&p);
                if let Some(ref p1) = next {
                    let node_cost = cost.measure(&p1, &p);
                    path.push((p, node_cost));
                    total_cost += node_cost;
                }
                path_node = next;
            }
            // fix to take ownership of points
            return Some(ShortestPath { path, total_cost });
        }

        let cf: &HashMap<Point, Point> = &came_from;
        let ns = NeighbourState {
            came_from: cf,
            current_point: &curr_node,
        };
        let n = neighbours.neighbours(&ns);
        for neighbour in n {
            let neighbour_cost = cost.measure(&curr_node, &neighbour);
            let neighbour_g_score = g_scores.get(&neighbour).unwrap_or(&INFINITY);
            let tentative_g_score = g_scores.get(&curr_node).unwrap_or(&INFINITY) + neighbour_cost;
            if tentative_g_score < *neighbour_g_score {
                came_from.insert(neighbour.clone(), curr_node.clone());
                g_scores.insert(neighbour.clone(), tentative_g_score);

                // distance to target
                let hueristic = heuristic.measure(&neighbour, &end);
                f_scores.insert(neighbour.clone(), tentative_g_score + hueristic);
                open_set.push(Candidate::new(
                    neighbour.clone(),
                    tentative_g_score + hueristic,
                ));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use crate::{
        cartesian::Plane,
        neighbour::{DirectNeighbours, TouchingNeighbours},
    };

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
    fn test_example() {
        let plane: &Plane = &(10, 10).into();
        let start = (0, 0).into();
        let end = (5, 5).into();

        let result = astar(
            start,
            end,
            &ManhattenDistance,
            &ManhattenDistance,
            &DirectNeighbours(&plane),
        );

        assert_eq!(10, result.unwrap().total_cost);
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

        let shortest_path = astar(start, end, &heuristic, &cost, &TouchingNeighbours(&plane));
        assert_eq!(1, shortest_path.unwrap().total_cost);
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

        let shortest_path = astar(start, end, &heuristic, &cost, &TouchingNeighbours(&plane));
        let tc = shortest_path.as_ref().unwrap().total_cost;
        assert_eq!(4, tc, "{:?}", shortest_path);
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
            vec![1, 1, INFINITY, INFINITY, INFINITY],
            vec![1, 1, 1, 1, 1],
            vec![INFINITY, INFINITY, INFINITY, INFINITY, 1],
            vec![1, 1, 1, 1, 1],
        ];

        let shortest_path = astar(start, end, &heuristic, &cost, &TouchingNeighbours(&plane));
        assert_eq!(12, shortest_path.unwrap().total_cost);
    }
}
