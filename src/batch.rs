use crate::errors::Result;
use crate::ll_client::Client;
use std::borrow::Cow;

pub struct Batch<'a> {
    client: &'a Client,
    write_key: Cow<'a, str>,
}

impl<'a> Batch<'a> {
    pub(crate) fn new<S>(client: &'a Client, writekey: S) -> Batch<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Batch {
            client,
            write_key: writekey.into(),
        }
    }

    pub fn enqueue(&self) -> Result<()> {
        Ok(())
    }

    pub fn flush(&self) -> Result<()> {
        Ok(())
    }

    pub fn close(self) -> Result<()> {
        Ok(())
    }
}
