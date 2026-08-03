//! Transport layer for Rift.
//!
//! This crate provides asynchronous byte-oriented communication over
//! network transports. It intentionally has no knowledge of the Rift
//! protocol, messages, or framing.
//!
//! Responsibilities:
//! - Establish connections
//! - Read and write byte streams
//! - Expose transport-related errors
//!
//! Non-responsibilities:
//! - Message framing
//! - Serialization
//! - Session management
//! - File transfer logic

mod connection;
mod writer;
mod reader;
mod error;
mod util;

pub mod quic;
pub mod tcp;

pub use connection::*;
pub use error::*;