use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "message too large")]
    MessageTooLarge,
}
