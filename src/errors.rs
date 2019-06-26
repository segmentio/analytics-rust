use crate::message::BatchMessage;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "error: {}", _0)]
    MessageTooLarge(String),
    #[fail(display = "error: maximum batch size reached")]
    MaxBatchSize(MaxBatchSize),

    #[fail(display = "channel full, message not sent: {:?}", message)]
    ChannelFull { message: BatchMessage },

    #[fail(display = "channel disconnected, message not sent: {:?}", message)]
    SendChannelDisconnected { message: BatchMessage },

    #[fail(display = "channel disconnected, cannot recv")]
    RecvChannelDisconnected,
}

#[derive(Debug)]
pub struct MaxBatchSize {
    pub message: BatchMessage,
}
