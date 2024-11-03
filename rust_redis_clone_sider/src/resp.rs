use std::fmt::Display;

use crate::resp_result::{RespError, RespResult};

#[derive(Debug, PartialEq)]
pub enum Resp {
    SimpleString(String),
}

impl Display for Resp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resp::SimpleString(string) => write!(f, "+{}\r\n", string),
        }
    }
}

/// Parses a simple string in the form `+VALUE\r\n`.
fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RespResult<Resp> {
    //
    resp_remove_type('+', buffer, index)?;

    let line = binary_extract_line_as_string(buffer, index)?;

    Ok(Resp::SimpleString(line))
}

/// Extract bytes from the buffer until a `\r` is reached.
fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RespResult<Vec<u8>> {
    //
    let mut output = Vec::new();

    // Prevent reading after the end of the buffer.
    if *index >= buffer.len() {
        return Err(RespError::OutOfBounds(*index));
    }

    // If there is not enough space for `\r\n`, the buffer is invalid.
    if buffer.len() - *index - 1 < 2 {
        *index = buffer.len();
        return Err(RespError::OutOfBounds(*index));
    }

    let mut previous_elem = buffer[*index].clone();
    let mut separator_found = false;
    let mut final_index = *index;

    // Scan the buffer, looking for `\r\n`.
    for &elem in buffer[*index..].iter() {
        final_index += 1;
        if elem == b'\n' && previous_elem == b'\r' {
            separator_found = true;
            break;
        }
        previous_elem = elem.clone();
    }

    // If the previous element is not `\n`, we are out of bounds.
    if !separator_found {
        *index = final_index;
        return Err(RespError::OutOfBounds(*index));
    }

    // Copy the bytes from the buffer to the output vector.
    output.extend_from_slice(&buffer[*index..final_index - 2]);

    // Update the index.
    *index = final_index;

    Ok(output)
}

/// Extracts bytes from the buffer until a `\r` is reached and converts them into a string.
pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RespResult<String> {
    //
    let line = binary_extract_line(buffer, index)?;
    Ok(String::from_utf8(line)?)
}

// Checks that the first character of a RESP buffer is the given one and removes it.
pub fn resp_remove_type(value: char, buffer: &[u8], index: &mut usize) -> RespResult<()> {
    //
    if buffer[*index] != value as u8 {
        return Err(RespError::WrongType);
    }
    *index += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index = 0;

        let output = parse_simple_string(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_binary_extract_line_empty_buffer() {
        let buffer = "".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_single_character() {
        let buffer = "O".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 1);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_index_too_advanced() {
        let buffer = "OK".as_bytes();
        let mut index = 1;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_no_separator() {
        let buffer = "OK".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_half_separator() {
        let buffer = "OK\r".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_incomplete_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RespError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index = 0;

        match binary_extract_line(buffer, &mut index) {
            Ok(line) => {
                assert_eq!(line, "OK".as_bytes());
                assert_eq!(index, 4);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_as_string() {
        let buffer = "OK\r\n".as_bytes();
        let mut index = 0;

        let output = binary_extract_line_as_string(buffer, &mut index).unwrap();

        assert_eq!(output, String::from("OK"));
        assert_eq!(index, 4);
    }

    #[test]
    fn test_resp_remove_type() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index = 0;

        resp_remove_type('+', buffer, &mut index).unwrap();

        assert_eq!(index, 1);
    }

    #[test]
    fn test_resp_remove_type_wrong_type() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index = 0;

        let err = resp_remove_type('+', buffer, &mut index).unwrap_err();

        assert_eq!(index, 0);
        assert_eq!(err, RespError::WrongType);
    }
}
