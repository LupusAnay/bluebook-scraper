use crate::{api, parser};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    IOError(std::io::Error),
    ApiError(api::Error),
    ParserError(parser::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::SerdeError(e)
    }
}

impl From<parser::Error> for Error {
    fn from(e: parser::Error) -> Self {
        Error::ParserError(e)
    }
}

impl From<api::Error> for Error {
    fn from(e: api::Error) -> Self {
        Error::ApiError(e)
    }
}
