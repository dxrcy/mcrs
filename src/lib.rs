//! **mcrs**: A Rust interface for a Minecraft server.
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
pub mod height_map;

mod block;
mod command;
mod connection;
mod coordinate;
mod response;

pub use block::Block;
pub use chunk::Chunk;
pub use connection::Connection;
pub use coordinate::Coordinate;
pub use height_map::HeightMap;
