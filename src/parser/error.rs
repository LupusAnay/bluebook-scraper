use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    CannotFindNode,
    WrongFormatNode,
    DataConvertError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::DataConvertError
    }
}
