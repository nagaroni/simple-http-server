use std::str::Utf8Error;
use super::method::{ MethodError, Method };
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub struct Request<'buf> {
    pub path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = std::str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

				let method : Method = method.parse()?;
				let mut query_string = None;
				if let Some(i) = path.find('?') {
					query_string = Some(&path[i + 1..]);
					path = &path[..i];
				}

				Ok(Self {
					path,
					query_string,
					method
				})
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

impl From<MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod
	}
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
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
