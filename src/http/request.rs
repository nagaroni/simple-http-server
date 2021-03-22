use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::io::Read;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request std::str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iterator = request.chars();
    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

impl From<Utf8Error> for ParseError {
    from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidRequest => "InvalidRequest"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
