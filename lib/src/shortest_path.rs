use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign},
    usize,
};

use hashbrown::HashMap;

use crate::grid::{Grid, GridPosition};

pub trait Neighbours<S> {
    fn neighbours(&self, state: &S) -> Vec<S>;
}

pub trait Cost<S, C>
where
    C: PartialOrd,
    C: Ord,
    C: Impossible,
{
    fn measure(&self, from: &S, to: &S) -> C;
}

pub trait Impossible {
    fn impossible() -> Self;
}

impl Impossible for i64 {
    fn impossible() -> Self {
        1000000
    }
}

impl Impossible for u64 {
    fn impossible() -> Self {
        1000000
    }
}

impl Impossible for usize {
    fn impossible() -> Self {
        1000000
    }
}

pub trait Heuristic<S, H>
where
    H: PartialOrd + Ord + Debug,
{
    fn predict(&self, from: &S) -> H;
}

pub struct Path<S, C> {
    pub path: Vec<(S, C)>,
    pub total_cost: C,
}

#[derive(Clone, Debug)]
struct Candidate<S, C> {
    state: S,
    cost: C,
}

impl<S, C> Candidate<S, C> {
    fn new(state: S, cost: C) -> Self {
        Candidate { state, cost }
    }
}

impl<S, C> PartialEq for Candidate<S, C>
where
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state)
    }
}

impl<S, C> Eq for Candidate<S, C>
where
    S: Eq,
    S: PartialEq,
{
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl<S, C> Ord for Candidate<S, C>
where
    C: Ord,
    S: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.state.cmp(&other.state))
    }
}

// `PartialOrd` needs to be implemented as well.
impl<S, C> PartialOrd for Candidate<S, C>
where
    C: Ord,
    S: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn astar<N, C, C1, H, S, F>(
    neighbours: &N,
    cost: &C,
    heuristic: &H,
    initial_state: S,
    end_state: F,
) -> Option<Path<S, C1>>
where
    N: Neighbours<S>,
    C: Cost<S, C1>,
    H: Heuristic<S, C1>,
    F: Fn(&S) -> bool,
    C1: Ord
        + Add<Output = C1>
        + AddAssign
        + Default
        + Copy
        + Impossible
        + PartialOrd
        + Display
        + Debug,
    S: Ord + Hash + Clone + Debug,
{
    let mut open_set: BinaryHeap<Candidate<S, C1>> = BinaryHeap::new();
    let mut came_from: HashMap<S, S> = HashMap::new();

    let mut g_scores: HashMap<S, C1> = HashMap::new();
    let mut f_scores: HashMap<S, C1> = HashMap::new();

    let start_f_score = heuristic.predict(&initial_state);
    open_set.push(Candidate::new(initial_state.clone(), start_f_score));
    g_scores.insert(initial_state.clone(), C1::default());
    f_scores.insert(initial_state.clone(), start_f_score);

    while let Some(curr_candid) = open_set.pop() {
        if end_state(&curr_candid.state) {
            let mut path: Vec<(S, C1)> = vec![];
            let mut path_node = Some(curr_candid.state);
            let mut total_cost = C1::default();

            while let Some(p) = path_node {
                let next = came_from.remove(&p);
                if let Some(ref p1) = next {
                    let node_cost = cost.measure(&p1, &p);
                    path.push((p, node_cost));
                    total_cost += node_cost;
                }
                path_node = next;
            }
            return Some(Path { path, total_cost });
        }

        let n = neighbours.neighbours(&curr_candid.state);
        for neighbour in n {
            let neighbour_cost = cost.measure(&curr_candid.state, &neighbour);

            let neighbour_g_score = *g_scores.get(&neighbour).unwrap_or(&C1::impossible());
            let tentative_g_score = *g_scores
                .get(&curr_candid.state)
                .unwrap_or(&C1::impossible())
                + neighbour_cost;

            if tentative_g_score < neighbour_g_score {
                came_from.insert(neighbour.clone(), curr_candid.state.clone());
                g_scores.insert(neighbour.clone(), tentative_g_score);

                // distance to target
                let h = heuristic.predict(&neighbour);
                f_scores.insert(neighbour.clone(), tentative_g_score + h);
                open_set.push(Candidate::new(neighbour.clone(), tentative_g_score + h));
            }
        }
    }

    None
}

pub struct ManhattenDistanceTo(pub GridPosition);

impl Heuristic<GridPosition, usize> for ManhattenDistanceTo {
    fn predict(&self, from: &GridPosition) -> usize {
        from.col.abs_diff(self.0.col) + from.row.abs_diff(self.0.row)
    }
}

impl Heuristic<GridPosition, i64> for ManhattenDistanceTo {
    fn predict(&self, from: &GridPosition) -> i64 {
        (from.col.abs_diff(self.0.col) + from.row.abs_diff(self.0.row)) as i64
    }
}

pub struct NonDiagonalNeighbours<'a, T>(pub &'a Grid<T>);

impl<'a, T> Neighbours<GridPosition> for NonDiagonalNeighbours<'a, T> {
    fn neighbours(&self, state: &GridPosition) -> Vec<GridPosition> {
        let mut result = Vec::new();

        if state.right().col < self.0.width() {
            result.push(state.right());
        }

        if state.row as i32 - 1 >= 0 {
            result.push(state.up());
        }

        if state.col as i32 - 1 >= 0 {
            result.push(state.left());
        }

        if state.row + 1 < self.0.height() {
            result.push(state.down());
        }
        result
    }
}

impl<T> Cost<GridPosition, T> for Grid<T>
where
    T: Impossible + Ord + Copy,
{
    fn measure(&self, _from: &GridPosition, to: &GridPosition) -> T {
        *self.at(to)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        grid::Grid,
        shortest_path::{GridPosition, ManhattenDistanceTo, NonDiagonalNeighbours},
        *,
    };

    use super::Neighbours;

    #[test]
    fn test_default_integers() {
        assert_eq!(0, usize::default());
        assert_eq!(0, i64::default());
    }

    #[test]
    fn test_grid_weights() {
        let s = "\
        1111111\n\
        1111111\n\
        1111111\n\
        1111111\
        "
        .parse::<Grid<usize>>()
        .unwrap();

        let neighbours = NonDiagonalNeighbours(&s);

        let end = GridPosition::new(6, 3);

        let initial_state = GridPosition::new(0, 0);
        let is_end_state = |&pos: &_| end == pos;

        let result = shortest_path::astar(
            &neighbours,
            &s,
            &ManhattenDistanceTo(end),
            initial_state,
            is_end_state,
        )
        .unwrap();

        assert_eq!(9, result.total_cost);
    }

    #[test]
    fn test_grid_weights_blocked() {
        let grid = "\
        1111111\n\
        1######\n\
        1#11111\n\
        111###1\
        "
        .parse::<Grid<char>>()
        .unwrap();

        let Grid { rows } = grid;
        let grid = Grid {
            rows: rows
                .into_iter()
                .map(|i| {
                    i.into_iter()
                        .map(|c| match c {
                            '#' => 10000000,
                            other => other.to_digit(10).unwrap() as usize,
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        };

        let neighbours = NonDiagonalNeighbours(&grid);

        let end = GridPosition::new(6, 3);

        let initial_state = GridPosition::new(0, 0);
        let is_end_state = |&pos: &_| end == pos;

        let result = shortest_path::astar(
            &neighbours,
            &grid,
            &ManhattenDistanceTo(end),
            initial_state,
            is_end_state,
        )
        .unwrap();

        assert_eq!(11, result.total_cost);
    }

    #[test]
    fn test_non_diagonal_neighbours() {
        let g: Grid<usize> = "\
        123456789\n\
        987654321\n\
        123456789\
        "
        .parse()
        .unwrap();

        let neighbours = NonDiagonalNeighbours(&g);

        let result: HashSet<_> = neighbours
            .neighbours(&GridPosition::new(8, 1))
            .into_iter()
            .collect();
        println!("{:?}", result);
        assert!(result.contains(&GridPosition::new(8, 2)));
        assert!(result.contains(&GridPosition::new(7, 1)));
        assert!(result.contains(&GridPosition::new(8, 0)));
    }
}
