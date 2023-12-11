use std::iter;

use crate::cartesian::Vector;

pub trait Distance {
    fn from_vector(v: Vector) -> Self;

    fn from_into_vector<T>(into: T) -> Self
    where
        T: Into<Vector> + Sized,
        Self: Sized,
    {
        Self::from_vector(into.into())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct StraightLineDistance(pub i64);

impl Distance for StraightLineDistance {
    fn from_vector(v: Vector) -> Self {
        let x_delta = (v.start.x - v.end.x).abs();
        let y_delta = (v.start.y - v.end.y).abs();
        let xy_delta = (v.start.x - v.end.y).abs();
        StraightLineDistance(x_delta.max(y_delta).max(xy_delta))
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct ManhattenDistance(pub i64);

impl Distance for ManhattenDistance {
    fn from_vector(v: Vector) -> Self {
        ManhattenDistance((v.start.x - v.end.x).abs() + (v.start.y - v.end.y).abs())
    }
}

impl iter::Sum for ManhattenDistance {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        ManhattenDistance(iter.map(|mh| mh.0).sum())
    }
}

// how do I add a blanked implementation for any Distance
impl From<Vector> for StraightLineDistance {
    fn from(value: Vector) -> Self {
        Self::from_vector(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::Point;

    use super::*;

    #[test]
    fn test_sum_for_manhatten_distance() {
        let v = vec![ManhattenDistance(1), ManhattenDistance(2), ManhattenDistance(3)];
        assert_eq!(ManhattenDistance(6), v.into_iter().sum())
    }

    #[test]
    fn test_straight_line_cost() {
        // ...
        // e..
        // s..
        let p1: Point = (0, 0).into();
        let p2: Point = (0, 1).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(1), v.into());

        // ...
        // s..
        // e..
        let p1: Point = (1, 0).into();
        let p2: Point = (0, 0).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(1), v.into());

        // ...
        // e..
        // .s.
        let p1: Point = (1, 0).into();
        let p2: Point = (0, 1).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(1), v.into());

        // ...
        // es.
        // ...
        let p1: Point = (1, 1).into();
        let p2: Point = (0, 1).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(1), v.into());

        // ...
        // se.
        // ...
        let p1: Point = (0, 1).into();
        let p2: Point = (1, 1).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(1), v.into());

        // ....
        // ...e
        // s...
        let p1: Point = (0, 0).into();
        let p2: Point = (3, 1).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(3), v.into());

        // ...e
        // ....
        // s...
        let p1: Point = (0, 0).into();
        let p2: Point = (3, 2).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(3), v.into());

        // ...e
        // ....
        // ....
        // s...
        let p1: Point = (0, 0).into();
        let p2: Point = (3, 3).into();
        let v: Vector = (p1, p2).into();
        assert_eq!(StraightLineDistance(3), v.into());
    }
}
