use crate::batcher::Batcher;
use crate::client::Client;
use crate::errors::Error as AnalyticsError;
use crate::http::HttpClient;
use crate::message::{BatchMessage, Context};
use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError};
use failure::Error;
use futures::future::Future;
use std::mem;
use uuid::Uuid;

pub struct AnalyticsBuilder<C> {
    write_key: String,
    client: C,
    channel_capacity: Option<usize>,
    context: Option<Context>,
}

impl AnalyticsBuilder<HttpClient> {
    pub fn new(write_key: String) -> AnalyticsBuilder<HttpClient> {
        Self::new_with_client(write_key, HttpClient::default())
    }
}

impl<C> AnalyticsBuilder<C> {
    pub fn new_with_client(write_key: String, client: C) -> AnalyticsBuilder<C> {
        AnalyticsBuilder {
            write_key,
            client,
            channel_capacity: None,
            context: None,
        }
    }

    pub fn channel_capacity<'a>(&'a mut self, channel_capacity: Option<usize>) -> &'a mut Self {
        self.channel_capacity = channel_capacity;
        self
    }

    pub fn context<'a>(&'a mut self, context: Context) -> &'a mut Self {
        self.context = Some(context);
        self
    }

    pub fn build(self) -> (Analytics, AnalyticsWorker<C>) {
        let (sender, receiver) = if let Some(cap) = self.channel_capacity {
            crossbeam_channel::bounded(cap)
        } else {
            crossbeam_channel::unbounded()
        };

        let batcher = Batcher::new(Uuid::new_v4().to_string(), self.context.clone());

        (
            Analytics { sender },
            AnalyticsWorker {
                receiver,
                batcher,
                client: self.client,
                write_key: self.write_key,
                context: self.context,
            },
        )
    }
}

#[derive(Clone)]
pub struct Analytics {
    sender: Sender<BatchMessage>,
}

impl Analytics {
    pub fn try_send(&self, msg: BatchMessage) -> Result<(), Error> {
        self.sender.try_send(msg).map_err(|err| {
            match err {
                TrySendError::Full(msg) => AnalyticsError::ChannelFull { message: msg },
                TrySendError::Disconnected(msg) => {
                    AnalyticsError::SendChannelDisconnected { message: msg }
                }
            }
            .into()
        })
    }
}

pub struct AnalyticsWorker<C> {
    receiver: Receiver<BatchMessage>,
    batcher: Batcher,
    client: C,
    write_key: String,
    context: Option<Context>,
}

impl<C: Client> AnalyticsWorker<C> {
    pub fn try_consume(&mut self) -> Result<(), Error> {
        let message = match self.receiver.try_recv() {
            Ok(message) => message,
            Err(TryRecvError::Disconnected) => {
                return Err(AnalyticsError::RecvChannelDisconnected.into());
            }
            Err(TryRecvError::Empty) => {
                return Ok(());
            }
        };

        if let Some(msg) = self.batcher.push(message)? {
            self.flush()?;
            self.batcher.push(msg)?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        let batcher = mem::replace(
            &mut self.batcher,
            Batcher::new(Uuid::new_v4().to_string(), self.context.clone()),
        );

        let to_flush = batcher.into_message();
        self.client.send(&self.write_key, &to_flush)?;
        Ok(())
    }

    //    pub fn run(&mut self) -> Future<()> {}
}
