//! **mcrs**: A Rust interface for a Minecraft server.
//!
//! Based on [mcpp](https://github.com/rozukke/mcpp).
//!
//! Requires a server running [ELCI](https://github.com/rozukke/elci).
//!
//! ```
//! # use mcrs::Connection;
//! let mut mc = Connection::new().unwrap();
//! mc.post_to_chat("Hello world!").unwrap();
//! ```

/// Types related to [`Chunk`]
pub mod chunk;
/// Types related to [`HeightMap`]
pub mod heights;

mod argument;
mod block;
mod connection;
mod coordinate;
mod response;
mod size;

pub use block::Block;
pub use chunk::Chunk;
pub use connection::Connection;
pub use coordinate::{Coordinate, Coordinate2D};
pub use heights::Heights;
pub use size::{Size, Size2D};
