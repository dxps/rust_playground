use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug, PartialEq)]
pub enum RespError {
    FromUtf8,
    IncorrectLength(RespLength),
    OutOfBounds(usize),
    ParseInt,
    Unknown,
    WrongType,
}

impl Display for RespError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FromUtf8 => write!(f, "Cannot convert to UTF-8"),
            Self::IncorrectLength(length) => write!(f, "Incorrect legth {}", length),
            Self::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            Self::ParseInt => write!(f, "Cannot parse string into integer"),
            Self::Unknown => write!(f, "Unknown format for RESP string"),
            Self::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<FromUtf8Error> for RespError {
    fn from(_: FromUtf8Error) -> Self {
        RespError::FromUtf8
    }
}

impl From<std::num::ParseIntError> for RespError {
    fn from(_: std::num::ParseIntError) -> Self {
        RespError::ParseInt
    }
}

pub type RespLength = i32;

pub type RespResult<T> = Result<T, RespError>;
