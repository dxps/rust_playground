use std::fmt::Display;

use crate::resp_result::{RespError, RespLength, RespResult};

#[derive(Debug, PartialEq)]
pub enum Resp {
    Array(Vec<Resp>),
    BulkString(String),
    Empty,
    SimpleString(String),
}

impl Display for Resp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Array(data) => {
                let mut output = String::from("*");
                output.push_str(format!("{}\r\n", data.len()).as_str());
                for elem in data.iter() {
                    output.push_str(elem.to_string().as_str());
                }
                output
            }
            // BulkString provides the length of the contained data as a prefix.
            Self::BulkString(string) => format!("${}\r\n{}\r\n", string.len(), string),
            Self::Empty => format!("$-1\r\n"),
            Self::SimpleString(string) => format!("+{}\r\n", string),
        };
        write!(f, "{}", output)
    }
}

fn parser_router(
    buffer: &[u8],
    index: &mut usize,
) -> Option<fn(&[u8], &mut usize) -> RespResult<Resp>> {
    match buffer[*index] {
        b'+' => Some(parse_simple_string),
        b'$' => Some(parse_bulk_string),
        b'*' => Some(parse_array),
        _ => None,
    }
}

pub fn bytes_to_resp(buffer: &[u8], index: &mut usize) -> RespResult<Resp> {
    match parser_router(buffer, index) {
        Some(parse_func) => {
            let result: Resp = parse_func(buffer, index)?;
            Ok(result)
        }
        None => Err(RespError::Unknown),
    }
}

/// Parses a simple string in the form `+VALUE\r\n`.
fn parse_simple_string(buffer: &[u8], index: &mut usize) -> RespResult<Resp> {
    //
    resp_remove_type('+', buffer, index)?;

    let line = binary_extract_line_as_string(buffer, index)?;

    Ok(Resp::SimpleString(line))
}

/// Parses a bulk string in the form `$LENGTH\r\nVALUE\r\n`.
fn parse_bulk_string(buffer: &[u8], index: &mut usize) -> RespResult<Resp> {
    //
    resp_remove_type('$', buffer, index)?;

    let length = resp_extract_length(buffer, index)?;
    if length == -1 {
        return Ok(Resp::Empty);
    }
    if length < -1 {
        return Err(RespError::IncorrectLength(length));
    }

    let bytes = binary_extract_bytes(buffer, index, length as usize)?;
    let data = String::from_utf8(bytes)?;

    // Increment the index to skip the `\r\n`.
    *index += 2;

    Ok(Resp::BulkString(data))
}

fn parse_array(buffer: &[u8], index: &mut usize) -> RespResult<Resp> {
    //
    resp_remove_type('*', buffer, index)?;

    let length = resp_extract_length(buffer, index)?;

    if length < 0 {
        return Err(RespError::IncorrectLength(length));
    }

    let mut data = Vec::new();

    for _ in 0..length {
        match parser_router(buffer, index) {
            Some(parse_func) => {
                let array_element = parse_func(buffer, index)?;
                data.push(array_element);
            }
            None => return Err(RespError::Unknown),
        }
    }

    Ok(Resp::Array(data))
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

/// Extract a given amount of bytes rom the buffer.
fn binary_extract_bytes(buffer: &[u8], index: &mut usize, length: usize) -> RespResult<Vec<u8>> {
    //
    let mut output = Vec::new();

    // Check if we are allowed to read length bytes.
    if *index + length >= buffer.len() {
        return Err(RespError::OutOfBounds(buffer.len()));
    }

    // Copy the bytes from the buffer to the output vector.
    output.extend_from_slice(&buffer[*index..*index + length]);

    // Update the index.
    *index += length;

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

/// Extracts a single line from a RESP buffer and interprets it as length.
/// The type used for the number is `RespLength`.
pub fn resp_extract_length(buffer: &[u8], index: &mut usize) -> RespResult<RespLength> {
    //
    let line = binary_extract_line_as_string(buffer, index)?;
    let length: RespLength = line.parse()?;
    Ok(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_resp_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index = 0;

        let output = bytes_to_resp(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_bytes_to_resp_bulk_string() {
        let buffer = b"$2\r\nOK\r\n";
        let mut index = 0;

        let output = bytes_to_resp(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::BulkString(String::from("OK")));
        assert_eq!(index, 8);
    }

    #[test]
    fn test_bytes_to_resp_unknown() {
        let buffer = "OK\r\n".as_bytes();
        let mut index = 0;

        let err = bytes_to_resp(buffer, &mut index).unwrap_err();

        assert_eq!(err, RespError::Unknown);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_bytes_to_resp_array() {
        let buffer = b"*2\r\n+OK\r\n$5\r\nVALUE\r\n";
        let mut index = 0;

        let output = bytes_to_resp(buffer, &mut index).unwrap();

        assert_eq!(
            output,
            Resp::Array(vec![
                Resp::SimpleString(String::from("OK")),
                Resp::BulkString(String::from("VALUE"))
            ])
        );
        assert_eq!(index, 20);
    }

    #[test]
    fn test_parse_simple_string() {
        let buffer = "+OK\r\n".as_bytes();
        let mut index = 0;

        let output = parse_simple_string(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::SimpleString(String::from("OK")));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_parse_bulk_string() {
        let buffer = b"$2\r\nOK\r\n";
        let mut index = 0;

        let output = parse_bulk_string(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::BulkString(String::from("OK")));
        assert_eq!(index, 8);
    }

    #[test]
    fn test_parse_bulk_string_empty() {
        let buffer = "$-1\r\n".as_bytes();
        let mut index: usize = 0;
        let output = parse_bulk_string(buffer, &mut index).unwrap();

        assert_eq!(output, Resp::Empty);
        assert_eq!(index, 5);
    }

    #[test]
    fn test_parse_bulk_string_wrong_type() {
        let buffer = "?2\r\nOK\r\n".as_bytes();
        let mut index: usize = 0;
        let error = parse_bulk_string(buffer, &mut index).unwrap_err();

        assert_eq!(error, RespError::WrongType);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_parse_bulk_string_unparsable_length() {
        let buffer = b"$wrong\r\nOK\r\n";
        let mut index: usize = 0;
        let error = parse_bulk_string(buffer, &mut index).unwrap_err();

        assert_eq!(error, RespError::ParseInt);
        assert_eq!(index, 8);
    }

    #[test]
    fn test_parse_bulk_string_negative_length() {
        let buffer = b"$-7\r\nOK\r\n";
        let mut index: usize = 0;
        let error = parse_bulk_string(buffer, &mut index).unwrap_err();

        assert_eq!(error, RespError::IncorrectLength(-7));
        assert_eq!(index, 5);
    }

    #[test]
    fn test_parse_bulk_string_data_too_short() {
        let buffer = b"$7\r\nOK\r\n";
        let mut index: usize = 0;
        let error = parse_bulk_string(buffer, &mut index).unwrap_err();

        assert_eq!(error, RespError::OutOfBounds(8));
        assert_eq!(index, 4);
    }

    #[test]
    fn test_parse_array() {
        let buffer = b"*2\r\n+OK\r\n$5\r\nVALUE\r\n";
        let mut index = 0;

        let output = parse_array(buffer, &mut index).unwrap();

        assert_eq!(
            output,
            Resp::Array(vec![
                Resp::SimpleString(String::from("OK")),
                Resp::BulkString(String::from("VALUE"))
            ])
        );
        assert_eq!(index, 20);
    }

    #[test]
    fn test_parse_array_invalid_length() {
        let buffer = b"*-1\r\nOK\r\n$5\r\nVALUE\r\n";
        let mut index = 0;

        let err = parse_array(buffer, &mut index).unwrap_err();

        assert_eq!(err, RespError::IncorrectLength(-1));
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
    fn test_binary_extract_bytes() {
        let buffer = b"SOMEBYTES";
        let mut index = 0;

        let output = binary_extract_bytes(buffer, &mut index, 6).unwrap();

        assert_eq!(output, "SOMEBY".as_bytes().to_vec());
        assert_eq!(index, 6);
    }

    #[test]
    fn test_binary_extract_bytes_out_of_bounds() {
        let buffer = b"SOMEBYTES";
        let mut index = 0;

        let err = binary_extract_bytes(buffer, &mut index, 10).unwrap_err();

        assert_eq!(err, RespError::OutOfBounds(9));
        assert_eq!(index, 0);
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
        let buffer = "*OK\r\n".as_bytes();
        let mut index = 0;

        let err = resp_remove_type('+', buffer, &mut index).unwrap_err();

        assert_eq!(index, 0);
        assert_eq!(err, RespError::WrongType);
    }
}
