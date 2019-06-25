use crate::message::BatchMessage;
use crossbeam_channel::{Sender, Receiver, TrySendError};
use failure::Error;
use crate::errors::Error as AnalyticsError;

pub struct AnalyticsBuilder {

}

pub struct Analytics {
    sender: Sender<BatchMessage>,
}

impl Analytics {
    pub fn try_send(&self, msg: BatchMessage) -> Result<(), Error> {
        self.sender.try_send(msg).map_err(|err| match err {
            TrySendError::Full(msg) => AnalyticsError::ChannelFull { message: msg },
            TrySendError::Disconnected(msg) => AnalyticsError::ChannelDisconnected { message: msg },
        }.into())
    }
}

pub struct AnalyticsWorker {
    receiver: Receiver<BatchMessage>,
}

impl AnalyticsWorker {
    pub fn try_consume(&self) -> Result<(), Error> {
        Ok(())
    }
}
