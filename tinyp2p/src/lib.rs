//! A tiny P2P library that has limited functionality but is easy to use.
//!
//! See the [examples](../examples/) directory for usage.

pub mod config;
pub mod error;

mod behaviour;
mod service;
mod transport;

pub use config::*;
pub use error::Error;
pub use service::{new, new_secret_key, Client, EventHandler, Server};

// Re-export libp2p types.
pub use libp2p::{Multiaddr, PeerId};
