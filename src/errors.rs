use failure::Fail;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "JSON error: {}", _0)]
    Json(#[fail(cause)] serde_json::error::Error),
    #[fail(display = "error: {}", _0)]
    MessageTooLarge(String),
    #[fail(display = "error: {}", _0)]
    MaxBatchSize(String),
    //    #[fail(display = "error: {}", _0)]
    //    Custom(String),
}

//impl From<ParseIntError> for Error {
//    fn from(error: ParseIntError) -> Self {
//        Error::InvalidNamespaceArrayIndex(error)
//    }
//}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::Json(error)
    }
}
