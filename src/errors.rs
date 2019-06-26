use failure::Fail;
use reqwest::Response;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "error: {}", _0)]
    MessageTooLarge(String),
}
