#![doc = include_str!("../README.md")]

mod auto_batcher;
mod batcher;
mod client;
mod errors;
mod http;
pub mod message;

pub use auto_batcher::AutoBatcher;
pub use batcher::Batcher;
pub use client::Client;
pub use errors::{Error, Result};
pub use http::HttpClient;
pub use message::Message;
