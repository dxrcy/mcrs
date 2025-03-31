use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::argument::Argument;
use crate::response::Response;
use crate::{Block, Chunk, Coordinate, Coordinate2D, Heights};

type Result<T> = io::Result<T>;

/// Connection for Minecraft server.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Default server address and port for [ELCI].
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    pub const DEFAULT_ADDRESS: &'static str = "127.0.0.1:4711";

    /// Create a new connection with the default server address.
    pub fn new() -> Result<Self> {
        Self::with_address::<&str>(Self::DEFAULT_ADDRESS)
    }

    /// Create a new connection with a specified server address.
    pub fn with_address<A>(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    /// Serialize and send a command to the server.
    fn send<'a>(
        &mut self,
        command: &'static str,
        arguments: impl AsRef<[Argument<'a>]>,
    ) -> Result<()> {
        self.stream.write_fmt(format_args!("{}(", command))?;
        for (i, arg) in arguments.as_ref().into_iter().enumerate() {
            if i > 0 {
                self.stream.write_fmt(format_args!(","))?;
            }
            self.stream.write_fmt(format_args!("{}", arg))?;
        }
        self.stream.write_fmt(format_args!(")\n"))?;
        Ok(())
    }

    /// Receive and deserialize the next response from the server.
    fn recv(&mut self) -> Result<Response> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(Response::new(buffer))
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

    /// Returns a [`Coordinate`] representing player position (block position of lower half of
    /// playermodel).
    pub fn get_player_position(&mut self) -> Result<Coordinate> {
        self.send("player.getPos", [])?;
        let response = self.recv()?;
        let coord = response.as_coordinate().expect("malformed server response");
        Ok(coord)
    }

    /// Returns the coordinate location of the block the player is standing on (i.e. tile).
    pub fn get_player_tile_position(&mut self) -> Result<Coordinate> {
        let mut coord = self.get_player_position()?;
        coord.y -= 1;
        Ok(coord)
    }

    /// Sets block at [`Coordinate`] to specified [`Block`].
    pub fn set_block(&mut self, location: impl Into<Coordinate>, block: Block) -> Result<()> {
        self.send(
            "world.setBlock",
            [
                Argument::Coordinate(location.into()),
                Argument::Block(block),
            ],
        )
    }

    /// Returns [`Block`] object from specified [`Coordinate`].
    pub fn get_block(&mut self, location: impl Into<Coordinate>) -> Result<Block> {
        self.send(
            "world.getBlockWithData",
            [Argument::Coordinate(location.into())],
        )?;
        let response = self.recv()?;
        let block = response.as_block().expect("malformed server response");
        Ok(block)
    }

    /// Sets a cuboid of blocks to all be the specified [`Block`], with the corners of the cuboid
    /// specified by [`Coordinate`]s `corner_a` and `corner_b` (in any order).
    pub fn set_blocks(
        &mut self,
        corner_a: impl Into<Coordinate>,
        corner_b: impl Into<Coordinate>,
        block: Block,
    ) -> Result<()> {
        self.send(
            "world.setBlocks",
            [
                Argument::Coordinate(corner_a.into()),
                Argument::Coordinate(corner_b.into()),
                Argument::Block(block),
            ],
        )
    }

    /// Returns a [`Chunk`] of the [`Block`]s of cuboid specified by [`Coordinate`]s `corner_a` and
    /// `corner_b` (in any order)
    pub fn get_blocks(
        &mut self,
        corner_a: impl Into<Coordinate>,
        corner_b: impl Into<Coordinate>,
    ) -> Result<Chunk> {
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
        let list = response.as_block_list().expect("malformed server response");
        let chunk = Chunk::new(corner_a, corner_b, list);
        Ok(chunk)
    }

    /// Returns the `y`-value of the highest solid block at the specified `x` and `z` coordinate
    ///
    /// **DO NOT USE FOR LARGE AREAS, IT WILL BE VERY SLOW** -- use [`get_heights`] instead.
    ///
    /// [`get_heights`]: Connection::get_heights
    pub fn get_height(&mut self, location: impl Into<Coordinate2D>) -> Result<i32> {
        self.send("world.getHeight", [Argument::Coordinate2D(location.into())])?;
        let response = self.recv()?;
        let height = response.as_integer().expect("malformed server response");
        Ok(height)
    }

    /// Provides a scaled option of the [`get_height`] call to allow for considerable performance
    /// gains.
    ///
    /// [`get_height`]: Connection::get_height
    pub fn get_heights(
        &mut self,
        corner_a: impl Into<Coordinate2D>,
        corner_b: impl Into<Coordinate2D>,
    ) -> Result<Heights> {
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
        let list = response.as_integer_list();
        let height_map = Heights::new(corner_a, corner_b, list);
        Ok(height_map)
    }
}
