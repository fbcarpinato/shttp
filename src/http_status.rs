use std::fmt;

pub enum HttpStatus {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl HttpStatus {
    fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::BadRequest => 400,
            HttpStatus::InternalServerError => 500,
        }
    }

    fn reason_phrase(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}

impl fmt::Display for HttpStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.code(), self.reason_phrase())
    }
}
