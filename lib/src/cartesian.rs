
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform {
    pub x: i64,
    pub y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl From<(i64, i64)> for Transform {
    fn from(value: (i64, i64)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        (x, y).into()
    }

    pub fn transform(&self, t: &Transform) -> Point {
        Point { x: self.x + t.x, y: self.y + t.y }
    }

    pub fn within(&self, plane: &Plane) -> bool {
        self.x >= plane.top_left.x && self.x <= plane.bottom_right.x && self.y >= plane.bottom_right.y && self.y <= plane.top_left.y
    }

    pub fn on(&self, v: &Vector) -> bool {
        let min_x = v.start.x.min(v.end.x);
        let max_x = v.start.x.max(v.end.x);
        let min_y = v.start.y.min(v.end.y);
        let max_y = v.start.y.max(v.end.y);
        let on_x = min_x <= self.x && max_x >= self.x;
        let on_y = min_y <= self.y && max_y >= self.y;        
        on_y && on_x
    }

    pub fn to(self, end: Point) -> Vector {
        Vector { start: self, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub start: Point,
    pub end: Point,
}

impl From<(Point, Point)> for Vector {
    fn from(value: (Point, Point)) -> Self {
        Self { start: value.0, end: value.1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plane {
    pub top_left: Point,
    pub bottom_right: Point
}

impl From<(i64, i64)> for Plane {
    fn from(value: (i64, i64)) -> Self {
        if value.0 < 0 {
            panic!("width must be be greated than 0 but was {}", value.0);
        }
        if value.1 < 0 {
            panic!("height must be be greated than 0 but was {}", value.1);
        }
        Plane { 
            top_left: (0, value.1 - 1).into(), 
            bottom_right: (value.0 - 1, 0).into()
        }
    }
}

impl Plane {
    pub fn width(&self) -> i64 {
        (self.top_left.x - self.bottom_right.x).abs()
    }

    pub fn height(&self) -> i64 {
        (self.top_left.y - self.bottom_right.y).abs()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_plane_width() {
        let p: Plane = (5, 11).into(); 
        assert_eq!(4, p.width())
    }

    #[test]
    fn test_plane_height() {
        let p: Plane = (5, 11).into(); 
        assert_eq!(10, p.height())
    }

    #[test]
    fn test_point_within_plane() {
        let p: Plane = (10, 10).into();

        assert_eq!(p.bottom_right, Point {
            x: 9,
            y: 0
        });
        assert_eq!(p.top_left, Point {
            x: 0,
            y: 9
        });
    }

    #[test]
    fn test_point_on_vector() {
        // #....
        // #....
        // #....
        // #....
        // #....

        let v: Vector = (
            (0, 0).into(),
            (0, 5).into(),
        ).into();

        for y in 0..=5 {
            let p: Point = (0, y).into();
            assert_eq!(true, p.on(&v));
        }

        // #....
        // #x...
        // #....
        // #....
        // #....
        let p: Point = (1, 1).into();
        assert_eq!(false, p.on(&v));
    }

    #[test]
    fn test_vector_ordering() {
        let v: Vector = (
            (498,6).into(),
            (496,6).into()
        ).into();

        let p: Point = (497, 6).into();
        assert_eq!(true, p.on(&v));
    }
}