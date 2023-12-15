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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

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

    pub fn flip(&self) -> Grid<T> {
        let mut rows = self.rows.clone();
        rows.reverse();
        Grid { rows }
    }
}

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
            .map(|l| {
                l.chars()
                    .filter_map(|c| T::from_char(c).ok())
                    .collect()
            })
            .collect();

        Ok(Grid { rows })
    }
}

impl<T> fmt::Display for Grid<T> 
where T: fmt::Display
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

#[test]
fn test_empty_line_chunks() {
    let text = &format!("first{}second{}third", EMPTY_LINE, EMPTY_LINE);

    let chunks: Vec<_> = empty_line_chunks(text).collect();
    assert_eq!(vec!["first", "second", "third"], chunks);
}

#[test]
fn test_transpose() {
    let g = Grid { 
        rows: vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
        ]
    };
    let expected = Grid {
        rows: vec![
            vec![1, 5],
            vec![2, 6],
            vec![3, 7],
            vec![4, 8],
        ]
    };

    assert_eq!(expected, g.transpose());
}

#[test]
fn test_flip() {
    let g = Grid { 
        rows: vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
        ]
    };
    let expected = Grid {
        rows: vec![
            vec![5, 6, 7, 8],
            vec![1, 2, 3, 4],            
        ]
    };

    println!("{}", expected);

    assert_eq!(expected, g.flip());
}