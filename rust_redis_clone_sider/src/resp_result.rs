use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug, PartialEq)]
pub enum RespError {
    FromUtf8,
    OutOfBounds(usize),
    WrongType,
}

impl Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RespError::FromUtf8 => write!(f, "Cannot convert to UTF-8"),
            RespError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RespError::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<FromUtf8Error> for RespError {
    fn from(_: FromUtf8Error) -> Self {
        RespError::FromUtf8
    }
}

pub type RespResult<T> = Result<T, RespError>;
