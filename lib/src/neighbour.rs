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