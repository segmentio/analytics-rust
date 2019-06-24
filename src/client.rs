use crate::message::Message;

pub trait Client<T, E> {
    fn send(msg: Message) -> Result<T, E>;
}
