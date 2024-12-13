use itertools::Itertools;
use std::error::Error;
use std::ops::Deref;
use std::{env, fmt};

#[derive(Debug, Clone)]
pub enum AocErrorType {
    InputDataNotFound,
    MissingArgument,
}
#[derive(Debug, Clone)]
pub struct AocError {
    err_type: AocErrorType,
}

impl AocError {
    pub fn new(err: AocErrorType) -> AocError {
        AocError { err_type: err }
    }
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AocError: {:?}", self.err_type)
    }
}

impl Error for AocError {}

pub fn get_arg1() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    args.get(1).cloned()
}

pub trait SimpleChar {
    fn as_i32(&self) -> i32;
}

impl SimpleChar for char {
    fn as_i32(&self) -> i32 {
        (*self as i32) - ('0' as i32)
    }
}

pub trait SimpleParse {
    fn get_i32(&self) -> i32;
    fn get_i64(&self) -> i64;
    fn to_pair(&self) -> (i32, i32);
}

impl SimpleParse for str {
    fn get_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }

    fn get_i64(&self) -> i64 {
        self.parse::<i64>().unwrap()
    }

    fn to_pair(&self) -> (i32, i32) {
        self.split_whitespace()
            .take(2)
            .map(|x| x.get_i32())
            .collect_tuple()
            .unwrap()
    }
}

pub trait MathHelpers {
    /// Round a float to a given amount of digits
    fn round_digits(&self, digits: usize) -> f64;
}

impl MathHelpers for f64 {
    fn round_digits(&self, digits: usize) -> f64 {
        let factor = f64::powf(10.0, digits as f64);
        (*self * factor).round() / factor
    }
}

// ------------------------------------------------------------------

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// ------------------------------------------------------------------
// Code for handling 2-dimensional structures of type

#[derive(PartialEq, Clone, Copy)]
pub struct Location {
    pub column: i32,
    pub row: i32,
}

pub struct Field<T>(Vec<Vec<T>>);

impl<T> Field<T> {
    pub fn new(v: Vec<Vec<T>>) -> Field<T> {
        Field(v)
    }

    pub fn at(&self, location: &Location) -> Option<T>
    where
        T: Copy,
    {
        if (location.row < 0) || (location.row as usize >= self.0.len()) {
            return None;
        }
        if (location.column < 0)
            || (location.column as usize >= self.0[location.row as usize].len())
        {
            return None;
        }
        Some(self.0[location.row as usize][location.column as usize])
    }

    pub fn at_mut(&mut self, location: &Location) -> Option<&mut T>
    where
        T: Copy,
    {
        if location.row >= 0 && (location.row as usize) < self.0.len() {
            if let Some(row) = self.0.get_mut(location.row as usize) {
                return row.get_mut(location.column as usize);
            }
        }
        None
    }

    /// Iterate through actually available neighbors.
    ///
    /// Returns an iterator that will return a tuple for each vertical and horizontal neighbor
    /// that is actually on the field, neighbors outside the field are ignored.
    /// The returned tuple contains the location of the neighbor and the value of that neighbor.
    ///
    /// To iterate over all neighbors, including those outside of the Field, use `all_neighbors()`.
    pub fn actual_neighbors<'a>(
        &'a self,
        location: &'a Location,
    ) -> impl Iterator<Item = (Location, T)> + 'a
    where
        T: Copy,
    {
        let neighbor_positions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        neighbor_positions.into_iter().filter_map(move |(dx, dy)| {
            let nx = location.column + dx;
            let ny = location.row + dy;
            let neighbor_location = Location {
                column: nx,
                row: ny,
            };

            self.0.get(ny as usize).and_then(|row| {
                row.get(nx as usize)
                    .cloned()
                    .map(|val| (neighbor_location, val))
            })
        })
    }

    pub fn all_neighbors<'a>(
        &'a self,
        location: &'a Location,
    ) -> impl Iterator<Item = (Location, Option<T>)> + 'a
    where
        T: Copy,
    {
        let neighbor_positions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        neighbor_positions.into_iter().map(move |(dx, dy)| {
            let nx = location.column + dx;
            let ny = location.row + dy;
            let neighbor_location = Location {
                column: nx,
                row: ny,
            };

            (
                neighbor_location,
                self.0
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize).cloned()),
            )
        })
    }
}

impl<T> Deref for Field<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ------------------------------------------------------------------

pub trait CharacterField {
    fn char_at(&self, x: i32, y: i32) -> Option<char>;
    fn has_position(&self, x: i32, y: i32) -> bool;

    fn with_char_at(&self, x: i32, y: i32, new_ch: char) -> Vec<String>;
}

impl CharacterField for Vec<&str> {
    fn char_at(&self, x: i32, y: i32) -> Option<char> {
        if (y < 0) || (y as usize >= self.len()) {
            return None;
        }
        if (x < 0) || (x as usize >= self[y as usize].len()) {
            return None;
        }
        Some(self[y as usize].as_bytes()[x as usize] as char)
    }

    fn has_position(&self, x: i32, y: i32) -> bool {
        (x >= 0) && (x < self[0].len() as i32) && (y >= 0) && (y < self.len() as i32)
    }

    fn with_char_at(&self, x: i32, y: i32, new_ch: char) -> Vec<String> {
        self.iter()
            .enumerate()
            .map(|(row, line)| {
                if row as i32 == y {
                    line.chars()
                        .enumerate()
                        .map(|(column, ch)| if column as i32 == x { new_ch } else { ch })
                        .collect()
                } else {
                    line.to_string()
                }
            })
            .collect()
    }
}
