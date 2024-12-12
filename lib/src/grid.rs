use core::fmt;
use std::str::FromStr;

use crate::{cartesian::Plane, input};

/// Utility for a grid of input. A common input type for aoc puzzles.
/// Supports a number of common utilities on grids of input.
///
/// Input would like like:
/// ```ignore
/// #..
/// .#.
/// ..#
/// ```
///
/// Parsing this input to a grid might look like:
///
/// ```
/// use aoclib::grid::Grid;
///
/// fn parse() {
///     let input = "\
///     #..\n\
///     .#.\n\
///     ..#
///     ";
///     let grid: Grid<char> = input.parse().unwrap();
///     println!("{}", grid);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// The number of columns within this `Grid`
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    /// The number of rows within this `Grid`
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn at(&self, pos: &GridPosition) -> &T {
        &self.rows[pos.row][pos.col]
    }

    pub fn up_from(&self, p: GridPosition) -> impl Iterator<Item = (GridPosition, &T)> {
        std::iter::successors(Some(p), move |&prev| {
            if prev.row == 0 {
                None
            } else {
                Some(prev.up())
            }
        }).map(move |p| (p, self.at(&p)))
    }

    pub fn down_from(&self, p: GridPosition) -> impl Iterator<Item = (GridPosition, &T)> {
        std::iter::successors(Some(p), move |&prev| {
            if prev.row < self.height() - 1 {
                Some(prev.down())
            } else {
                None
            }
        }).map(move |p| (p, self.at(&p)))
    }

    pub fn left_from(&self, p: GridPosition) -> impl Iterator<Item = (GridPosition, &T)> {
        std::iter::successors(Some(p), move |&prev| {
            if prev.col == 0 {
                None
            } else {
                Some(prev.left())
            }
        }).map(move |p| (p, self.at(&p)))
    }

    pub fn right_from(&self, p: GridPosition) -> impl Iterator<Item = (GridPosition, &T)> {
        std::iter::successors(Some(p), move |&prev| {
            if prev.col < self.width() - 1 {
                Some(prev.right())
            } else {
                None                
            }
        }).map(move |p| (p, self.at(&p)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridPosition {
    pub col: usize,
    pub row: usize,
}

impl GridPosition {
    pub fn new(col: usize, row: usize) -> Self {
        Self { row, col }
    }

    pub fn up(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl<T> From<&Grid<T>> for Plane {
    fn from(value: &Grid<T>) -> Self {
        (value.width() as i64, value.height() as i64).into()
    }
}

impl<T> Grid<T>
where
    T: Default,
    T: Clone,
{
    pub fn transpose(&self) -> Grid<T> {
        let rows = self.rows.clone();
        let mut cols = vec![vec![T::default(); rows.len()]; rows[0].len()];

        for col in 0..rows[0].len() {
            for row in 0..rows.len() {
                cols[col][row] = rows[row][col].clone();
            }
        }

        Grid { rows: cols }
    }
}

/// If the type contained within your `Grid` behaves differently
/// if the grid is flipped, then you should implement this
/// to perform that flip.
///
/// For example, a grid of mirrors:
/// ```ignore
/// . . .
/// . / .
/// . . .
/// ```
///
/// where light travelling from top to bottom, reflects
/// the on the middle mirror to the left.
///
/// When flipped:
/// ```ignore
/// . . .
/// . \ .
/// . . .
/// ```
///
/// light now travelling bottom to top, should still
/// reflect to the left. The flip behavour depends on the type.
pub trait Flip {
    fn flip(&self) -> Self;
}

impl<T> Flip for Grid<T>
where
    T: Flip,
    T: Clone,
{
    fn flip(&self) -> Self {
        let mut rows = self.rows.clone();
        rows.reverse();
        Grid { rows }
    }
}

/// Required `trait` for a generic `Grid`
///
/// implement this to transform the individial grid characters to
/// your desired type. Typically this would be an enum but can be
/// whatever suits your use-case.
///
/// eg:
/// ```
/// use aoclib::grid::FromChar;
///
/// enum Tile {
///     Space,
///     DiagonalRight,
/// }
///
/// impl FromChar for Tile {
///     type Err = String;
///
///     fn from_char(c: char) -> Result<Self, Self::Err> {
///         match c {
///             '.' => Ok(Tile::Space),
///             '/' => Ok(Tile::DiagonalRight),
///             _ => Err("no mapping".to_owned()),
///          }
///     }
/// }
/// ```
///
pub trait FromChar: Sized {
    type Err;

    fn from_char(c: char) -> Result<Self, Self::Err>;
}

impl FromChar for char {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        Ok(c)
    }
}

impl FromChar for u32 {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c.to_digit(10) {
            Some(d) => Ok(d),
            None => Err(format!("bad digit {c}")),
        }
    }
}

impl FromChar for usize {
    type Err = String;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        match c.to_digit(10) {
            Some(d) => Ok(d as usize),
            None => Err(format!("bad digit {c}")),
        }
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromChar,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|l| l.chars().filter_map(|c| T::from_char(c).ok()).collect())
            .collect();

        Ok(Grid { rows })
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = fmt::Result::Ok(());
        let plane: Plane = self.into();
        let width = plane.width();
        let height = plane.height();
        for row in 0..height {
            for col in 0..width {
                let item = &self.rows[row as usize][col as usize];
                result = result.and_then(|_| item.fmt(f));
            }
            result = result.and_then(|_| f.write_str(input::NEW_LINE));
        }
        result
    }
}

impl Flip for u32 {
    fn flip(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {

    use crate::{grid::Grid, input::*};

    use super::{Flip, GridPosition};

    #[test]
    fn test_empty_line_chunks() {
        let text = &format!("first{}second{}third", EMPTY_LINE, EMPTY_LINE);

        let chunks: Vec<_> = empty_line_chunks(text).collect();
        assert_eq!(vec!["first", "second", "third"], chunks);
    }

    #[test]
    fn test_transpose() {
        let g = Grid {
            rows: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]],
        };
        let expected = Grid {
            rows: vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8]],
        };

        assert_eq!(expected, g.transpose());
    }

    impl Flip for i32 {
        fn flip(&self) -> Self {
            self.clone()
        }
    }

    #[test]
    fn test_flip() {
        let g = Grid {
            rows: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]],
        };
        let expected = Grid {
            rows: vec![vec![5, 6, 7, 8], vec![1, 2, 3, 4]],
        };

        println!("{}", expected);

        assert_eq!(expected, g.flip());
    }

    #[test]
    fn test_iter_right() {
        let g = Grid {
            rows: vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ]
        };
        let right: Vec<_> = g.right_from(GridPosition::new(0, 0))
            .map(|(_, b)| b)
            .copied()
            .collect();
        assert_eq!(vec![1,2,3], right);
    }

    #[test]
    fn test_iter_left() {
        let g = Grid {
            rows: vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ]
        };
        let right: Vec<_> = g.left_from(GridPosition::new(2, 0))
            .map(|(_, b)| b)
            .copied()
            .collect();
        assert_eq!(vec![3,2,1], right);
    }

    #[test]
    fn test_iter_up() {
        let g = Grid {
            rows: vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ]
        };
        let right: Vec<_> = g.up_from(GridPosition::new(0, 2))
            .map(|(_, b)| b)
            .copied()
            .collect();
        assert_eq!(vec![7,4,1], right);
    }

    #[test]
    fn test_iter_down() {
        let g = Grid {
            rows: vec![
                vec![1,2,3],
                vec![4,5,6],
                vec![7,8,9],
            ]
        };
        let right: Vec<_> = g.down_from(GridPosition::new(1, 0))
            .map(|(_, b)| b)
            .copied()
            .collect();
        assert_eq!(vec![2,5,8], right);
    }
}
