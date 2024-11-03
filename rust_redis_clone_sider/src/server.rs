use std::fmt::Display;

use crate::resp::Resp;

#[derive(Debug, PartialEq)]
pub enum ServerError {
    CommandError,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandError => write!(f, "Error while processing!"),
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub fn process_request(request: Resp) -> ServerResult<Resp> {
    //
    let elements = match request {
        Resp::Array(v) => v,
        _ => return Err(ServerError::CommandError),
    };

    let mut cmd = Vec::new();
    for elem in elements.iter() {
        match elem {
            Resp::BulkString(v) => cmd.push(v),
            _ => return Err(ServerError::CommandError),
        }
    }

    match cmd[0].to_lowercase().as_str() {
        "ping" => Ok(Resp::SimpleString("PONG".to_string())),
        "echo" => Ok(Resp::BulkString(cmd[1].to_string())),
        _ => {
            return Err(ServerError::CommandError);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_request_ping() {
        //
        let request = Resp::Array(vec![Resp::BulkString("PING".to_string())]);

        let output = process_request(request).unwrap();

        assert_eq!(output, Resp::SimpleString("PONG".to_string()));
    }

    #[test]
    fn test_process_request_echo() {
        //
        let request = Resp::Array(vec![
            Resp::BulkString("ECHO".to_string()),
            Resp::BulkString("42".to_string()),
        ]);

        let output = process_request(request).unwrap();

        assert_eq!(output, Resp::BulkString("42".to_string()));
    }

    #[test]
    fn test_process_request_not_array() {
        //
        let request = Resp::BulkString("PING".to_string());

        let err = process_request(request).unwrap_err();

        assert_eq!(err, ServerError::CommandError);
    }

    #[test]
    fn test_process_request_not_bulkstring() {
        //
        let request = Resp::Array(vec![Resp::SimpleString("PING".to_string())]);

        let err = process_request(request).unwrap_err();

        assert_eq!(err, ServerError::CommandError);
    }
}
