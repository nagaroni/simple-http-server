use std::str::FromStr;

pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            "CONNECT" => Ok(Self::CONNECT),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
