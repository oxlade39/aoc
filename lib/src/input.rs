use core::fmt;
use std::str::FromStr;

use crate::cartesian::Plane;

#[cfg(windows)]
const NEW_LINE: &'static str = "\r\n";

#[cfg(not(windows))]
const NEW_LINE: &'static str = "\n";

#[cfg(windows)]
const EMPTY_LINE: &'static str = "\r\n\r\n";

#[cfg(not(windows))]
const EMPTY_LINE: &'static str = "\n\n";

pub fn empty_line_chunks<'a>(input: &'a str) -> std::str::Split<'a, &'a str> {
    input.split(EMPTY_LINE)
}

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
/// use aoclib::input::Grid;
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
/// use aoclib::input::FromChar;
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
            result = result.and_then(|_| f.write_str(NEW_LINE));
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::input::*;

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
}
