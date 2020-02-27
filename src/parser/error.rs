use crate::api;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    CannotFindNode,
    WrongFormatNode,
    DataConvertError,
    NetworkSubpageError(api::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::DataConvertError
    }
}

impl From<api::Error> for Error {
    fn from(e: api::Error) -> Self {
        Error::NetworkSubpageError(e)
    }
}
