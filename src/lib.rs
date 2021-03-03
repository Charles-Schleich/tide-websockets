//! it's websockets, for tide!
//!
//! see [`WebSocket`] for examples and usage

#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_docs,
    unreachable_pub,
    missing_copy_implementations,
    unused_qualifications
)]

mod handler;

pub use handler::WebSocket;

pub use async_tungstenite;
pub use async_tungstenite::tungstenite::{self, Error, Message};
