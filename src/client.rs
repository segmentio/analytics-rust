use crate::message::Message;

pub trait Client<T, E> {
    fn send(&self, write_key: &str, msg: &Message) -> Result<T, E>;
}
