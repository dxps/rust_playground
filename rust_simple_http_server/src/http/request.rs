use super::method::Method;
use std::convert::TryFrom; // adding the trait into the scope

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

/// Implementation of the standard TryFrom trait for the Request.
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(
        buf: &[u8],
    ) -> std::result::Result<Self, <Self as std::convert::TryFrom<&[u8]>>::Error> {
        todo!()
    }
}
