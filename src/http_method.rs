use std::fmt;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
}

pub struct ParseHttpMethodError;

impl fmt::Display for ParseHttpMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP method")
    }
}

impl FromStr for HttpMethod {
    type Err = ParseHttpMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PUT" => Ok(HttpMethod::PUT),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(ParseHttpMethodError),
        }
    }
}
