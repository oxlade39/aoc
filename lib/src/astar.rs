use std::collections::{BTreeSet, HashMap};

use crate::cartesian::{Point, Plane};

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
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

trait Cost {
    fn measure(&self, from: &Point, to: &Point) -> i64;
}

#[derive(Debug, PartialEq)]
struct ShortestPath {
    path: Vec<(Point, i64)>,
    total_cost: i64
}

fn astar<H, C>(
    plane: &Plane, 
    start: Point, 
    end: Point,
    heuristic: H,
    cost: C,
) -> ShortestPath 
where H: Hueristic, C: Cost
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
        for neighbour in curr_node.neighbours(plane) {
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
        let plane = &(10, 10).into();
        let start = (0, 0).into();
        let end = (5, 5).into();

        let result = astar(
            plane, 
            start, 
            end, 
            ManhattenDistance, 
            ManhattenDistance,
        );

        println!("shortest path: {:?}", result);
    }

}