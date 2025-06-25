use std::io::{self, Write};

use crate::argument::Argument;
use crate::chunk::ChunkStream;
use crate::heights::HeightsStream;
use crate::response::{BufReader, ResponseStream};
use crate::{Block, Chunk, Coordinate, Coordinate2D, Heights, Result};

#[cfg(not(feature = "uds"))]
use std::net::TcpStream;
#[cfg(feature = "uds")]
use std::os::unix::net::UnixStream;

// `pub(crate)` so that `ResponseStream` can use it without being generic
#[cfg(not(feature = "uds"))]
pub(crate) type Stream = TcpStream;
#[cfg(feature = "uds")]
pub(crate) type Stream = UnixStream;

/// Connection for Minecraft server.
#[derive(Debug)]
pub struct Connection {
    stream: Stream,
    reader: BufReader<Stream>,
}

// TODO(feat): Add context to errors?
// TODO(doc): Rewrite documentation for functionality methods

impl Connection {
    /// Default address and port for [ELCI] server.
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    #[cfg(not(feature = "uds"))]
    pub const DEFAULT_ADDRESS: &str = "127.0.0.1:4711";

    /// Default unix socket path for [ELCI] server.
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    #[cfg(feature = "uds")]
    pub const DEFAULT_PATH: &str = "/tmp/elci-proxy";

    /// Create a new connection with the ([default server address]).
    ///
    /// [default server address]: Self::DEFAULT_ADDRESS
    #[cfg(not(feature = "uds"))]
    pub fn new() -> io::Result<Self> {
        Self::from_stream(TcpStream::connect(Self::DEFAULT_ADDRESS)?)
    }

    /// Create a new connection with the ([default server path]).
    ///
    /// [default server path]: Self::DEFAULT_PATH
    #[cfg(feature = "uds")]
    pub fn new() -> io::Result<Self> {
        Self::from_stream(UnixStream::connect(Self::DEFAULT_PATH)?)
    }

    /// Create a new connection with a specified server address.
    #[cfg(not(feature = "uds"))]
    pub fn with_address(addr: impl std::net::ToSocketAddrs) -> io::Result<Self> {
        Self::from_stream(TcpStream::connect(addr)?)
    }

    /// Create a new connection with a specified server path.
    #[cfg(feature = "uds")]
    pub fn with_path(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        Self::from_stream(UnixStream::connect(path)?)
    }

    /// Create a new connection from a given [`Stream`].
    fn from_stream(stream: Stream) -> io::Result<Self> {
        let reader = BufReader::new(stream.try_clone()?);
        Ok(Self { stream, reader })
    }

    /// Serialize and send a command to the server.
    fn send<'a>(
        &mut self,
        command: &'static str,
        arguments: impl AsRef<[Argument<'a>]>,
    ) -> Result<()> {
        self.stream.write_fmt(format_args!("{}(", command))?;
        for (i, arg) in arguments.as_ref().iter().enumerate() {
            if i > 0 {
                self.stream.write_fmt(format_args!(","))?;
            }
            self.stream.write_fmt(format_args!("{}", arg))?;
        }
        self.stream.write_fmt(format_args!(")\n"))?;
        Ok(())
    }

    /// Creates a [`ResponseStream`] to read from the server.
    fn recv(&mut self) -> Result<ResponseStream> {
        ResponseStream::new(&mut self.reader)
    }

    /// Sends a message to the in-game chat.
    ///
    /// Does **not** require that a player has joined.
    pub fn post_to_chat(&mut self, message: impl AsRef<str>) -> Result<()> {
        self.send("chat.post", [Argument::String(message.as_ref())])
    }

    /// Performs an in-game Minecraft command.
    ///
    /// Players have to exist on the server and should be server operators (default with [ELCI]).
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    pub fn do_command(&mut self, command: impl AsRef<str>) -> Result<()> {
        self.send("player.doCommand", [Argument::String(command.as_ref())])
    }

    /// Returns a [`Coordinate`] representing player position (block position of lower half of
    /// playermodel).
    pub fn get_player_position(&mut self) -> Result<Coordinate> {
        self.send("player.getPos", [])?;
        let mut response = self.recv()?;
        let coord = response.final_coordinate()?;
        Ok(coord)
    }

    /// Returns the coordinate location of the block the player is standing on (i.e. tile).
    pub fn get_player_tile_position(&mut self) -> Result<Coordinate> {
        let mut coord = self.get_player_position()?;
        coord.y -= 1;
        Ok(coord)
    }

    /// Sets player position (block position of lower half of playermodel) to specified
    /// [`Coordinate`].
    pub fn set_player_position(&mut self, position: impl Into<Coordinate>) -> Result<()> {
        self.send("player.setPos", [Argument::Coordinate(position.into())])
    }

    /// Sets player position to be one above specified tile (i.e. tile = block player is standing
    /// on).
    pub fn set_player_tile_position(&mut self, position: impl Into<Coordinate>) -> Result<()> {
        let mut position = position.into();
        position.y += 1;
        self.set_player_position(position)
    }

    /// Returns [`Block`] object from specified [`Coordinate`].
    pub fn get_block(&mut self, location: impl Into<Coordinate>) -> Result<Block> {
        self.send(
            "world.getBlockWithData",
            [Argument::Coordinate(location.into())],
        )?;
        let mut response = self.recv()?;
        let block = response.final_block()?;
        Ok(block)
    }

    /// Sets block at [`Coordinate`] to specified [`Block`].
    pub fn set_block(
        &mut self,
        location: impl Into<Coordinate>,
        block: impl Into<Block>,
    ) -> Result<()> {
        self.send(
            "world.setBlock",
            [
                Argument::Coordinate(location.into()),
                Argument::Block(block.into()),
            ],
        )
    }

    /// Returns the `y`-value of the highest solid block at the specified `x` and `z` coordinate
    ///
    /// **DO NOT USE FOR LARGE AREAS, IT WILL BE VERY SLOW** -- use [`get_heights`] instead.
    ///
    /// [`get_heights`]: Connection::get_heights
    pub fn get_height(&mut self, location: impl Into<Coordinate2D>) -> Result<i32> {
        self.send("world.getHeight", [Argument::Coordinate2D(location.into())])?;
        let mut response = self.recv()?;
        let height = response.final_i32()?;
        Ok(height)
    }

    /// Sets a cuboid of blocks to all be the specified [`Block`], with the corners of the cuboid
    /// specified by [`Coordinate`]s `corner_a` and `corner_b` (in any order).
    pub fn set_blocks(
        &mut self,
        corner_a: impl Into<Coordinate>,
        corner_b: impl Into<Coordinate>,
        block: impl Into<Block>,
    ) -> Result<()> {
        self.send(
            "world.setBlocks",
            [
                Argument::Coordinate(corner_a.into()),
                Argument::Coordinate(corner_b.into()),
                Argument::Block(block.into()),
            ],
        )
    }

    /// Returns a [`Chunk`] structure of the [`Block`]s in cuboid specified by [`Coordinate`]s
    /// `corner_a` and `corner_b` (in any order).
    ///
    /// Reads entire response and allocates [`Chunk`] structure. To read response as a stream, use
    /// [`get_blocks_stream`] instead.
    ///
    /// [`get_blocks_stream`]: Connection::get_blocks_stream
    // TODO(rename): get_chunk
    pub fn get_blocks(
        &mut self,
        corner_a: impl Into<Coordinate>,
        corner_b: impl Into<Coordinate>,
    ) -> Result<Chunk> {
        self.get_blocks_stream(corner_a, corner_b)?.collect()
    }

    /// Returns a [`Chunk`] structure of the [`Block`]s in cuboid specified by [`Coordinate`]s
    /// `corner_a` and `corner_b` (in any order).
    ///
    /// Reads response as a stream to avoid unneccessary allocation. See also: [`get_blocks`].
    ///
    /// [`get_blocks`]: Connection::get_blocks
    // TODO(rename): get_chunk_stream
    pub fn get_blocks_stream(
        &mut self,
        corner_a: impl Into<Coordinate>,
        corner_b: impl Into<Coordinate>,
    ) -> Result<ChunkStream> {
        let corner_a = corner_a.into();
        let corner_b = corner_b.into();
        self.send(
            "world.getBlocksWithData",
            [
                Argument::Coordinate(corner_a),
                Argument::Coordinate(corner_b),
            ],
        )?;
        let response = self.recv()?;
        let chunk = ChunkStream::new(corner_a, corner_b, response);
        Ok(chunk)
    }

    /// Returns a [`Heights`] structure of y-values in rectangle specified by [`Coordinate2D`]s
    /// `corner_a` and `corner_b` (in any order).
    ///
    /// Reads entire response and allocates [`Heights`] structure. To read response as a stream, use
    /// [`get_heights_stream`] instead.
    ///
    /// [`get_heights_stream`]: Connection::get_heights_stream
    pub fn get_heights(
        &mut self,
        corner_a: impl Into<Coordinate2D>,
        corner_b: impl Into<Coordinate2D>,
    ) -> Result<Heights> {
        self.get_heights_stream(corner_a, corner_b)?.collect()
    }

    /// Returns a [`Heights`] structure of y-values in rectangle specified by [`Coordinate2D`]s
    /// `corner_a` and `corner_b` (in any order).
    ///
    /// Reads response as a stream to avoid unneccessary allocation. See also: [`get_heights`].
    ///
    /// [`get_heights`]: Connection::get_heights
    pub fn get_heights_stream(
        &mut self,
        corner_a: impl Into<Coordinate2D>,
        corner_b: impl Into<Coordinate2D>,
    ) -> Result<HeightsStream> {
        let corner_a = corner_a.into();
        let corner_b = corner_b.into();
        self.send(
            "world.getHeights",
            [
                Argument::Coordinate2D(corner_a),
                Argument::Coordinate2D(corner_b),
            ],
        )?;
        let response = self.recv()?;
        let heights = HeightsStream::new(corner_a, corner_b, response);
        Ok(heights)
    }
}
