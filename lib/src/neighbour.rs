use crate::cartesian::{Point, Transform, Plane};

pub trait Neighbours {
    fn neighbours(&self, p: &Point) -> Vec<Point>;
}

pub struct DirectNeighbours<'a>(pub &'a Plane);
pub struct TouchingNeighbours<'a>(pub &'a Plane);

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
        [
            (-1, 0),
            (1, 0),
            (0, 1),
            (0, -1),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ]
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{cartesian::{Point, Plane}, neighbour::{DirectNeighbours, Neighbours, TouchingNeighbours}};


    #[test]
    fn test_direct_neighbours_at_edge() {
        let p: Point = (0, 0).into();
        let plane: Plane = (10, 10).into();
        let neighbours = DirectNeighbours(&plane);
        let n = neighbours.neighbours(&p);

        let expected: Vec<Point> = vec![(1, 0).into(), (0, 1).into()];
        assert_eq!(n, expected);
    }

    #[test]
    fn test_touching_neighbours_includes_diagonals() {
        // ...
        // .P.
        // ...
        let plane: Plane = (3, 3).into();
        let p: Point = (1, 1).into();

        let expected: HashSet<Point> = HashSet::from_iter(
            vec![
                (0, 2),
                (1, 2),
                (2, 2),
                (0, 1),
                (2, 1),
                (0, 0),
                (1, 0),
                (2, 0),
            ]
            .into_iter()
            .map(|p| p.into()),
        );

        let neighbours = TouchingNeighbours(&plane);
        let n: HashSet<Point> = HashSet::from_iter(neighbours.neighbours(&p));

        assert_eq!(n, expected);
    }
}