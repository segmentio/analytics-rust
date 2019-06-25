use crate::message::BatchMessage;
use failure::Fail;
use std::io;
//use url::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error: {}", _0)]
    Io(#[fail(cause)] io::Error),
    #[fail(display = "JSON error: {}", _0)]
    Json(#[fail(cause)] serde_json::error::Error),
    #[fail(display = "error: {}", _0)]
    MessageTooLarge(String),
    #[fail(display = "error: maximum batch size reached")]
    MaxBatchSize(MaxBatchSize),
    //    #[fail(display = "URL error: {}", _0)]
    //    Url(#[fail(cause)] url::ParseError),
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

//impl From<url::ParseError> for Error {
//    fn from(error: url::ParseError) -> Self {
//        Error::Url(error)
//    }
//}

#[derive(Debug)]
pub struct MaxBatchSize {
    pub message: BatchMessage,
}
