use itertools::Itertools;
use std::error::Error;
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
