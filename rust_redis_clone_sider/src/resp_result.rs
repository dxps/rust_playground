use std::fmt::Display;

#[derive(Debug)]
pub enum RespError {
    OutOfBounds(usize),
}

impl Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
        }
    }
}

pub type RespResult<T> = Result<T, RespError>;
