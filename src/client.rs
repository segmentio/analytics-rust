use crate::{Message, Result};

#[async_trait::async_trait]
pub trait Client {
    async fn send(&self, write_key: String, msg: Message) -> Result<()>;
}
