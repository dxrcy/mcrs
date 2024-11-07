use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{
    command::Command, height_map::HeightMap, response::Response, Block, Chunk, Coordinate,
};

type Result<T> = io::Result<T>;

/// Connection for Minecraft server
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Default server address and port for [ELCI]
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    pub const DEFAULT_ADDRESS: &'static str = "127.0.0.1:4711";

    /// Create a new connection with the default server address
    pub fn new() -> Result<Self> {
        Self::with_address::<&str>(Self::DEFAULT_ADDRESS)
    }

    /// Create a new connection with a specified server address
    pub fn with_address<A>(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    /// Serialize and send a command to the server
    fn send(&mut self, command: Command) -> Result<()> {
        self.stream.write_all(command.build().as_bytes())?;
        Ok(())
    }

    /// Receive and deserialize a response from the server
    fn recv(&mut self) -> Result<Response> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(Response::new(buffer))
    }

    /// Sends a message to the in-game chat, does not require a joined player
    pub fn post_to_chat(&mut self, message: impl AsRef<str>) -> Result<()> {
        self.send(Command::new("chat.post").arg_string(message))
    }

    /// Performs an in-game Minecraft command. Players have to exist on the
    /// server and should be server operators (default with [ELCI])
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    pub fn do_command(&mut self, command: impl AsRef<str>) -> Result<()> {
        self.send(Command::new("player.doCommand").arg_string(command))
    }

    /// Sets player position (block position of lower half of playermodel) to
    /// specified [`Coordinate`]
    pub fn set_player_position(&mut self, position: impl Into<Coordinate>) -> Result<()> {
        self.send(Command::new("player.setPos").arg_coordinate(position.into()))
    }

    /// Sets player position to be one above specified tile (i.e. tile = block
    /// player is standing on)
    pub fn set_player_tile_position(&mut self, position: impl Into<Coordinate>) -> Result<()> {
        let mut position = position.into();
        position.y += 1;
        self.set_player_position(position)
    }

    /// Returns a [`Coordinate`] representing player position (block position of
    /// lower half of playermodel)
    pub fn get_player_position(&mut self) -> Result<Coordinate> {
        self.send(Command::new("player.getPos"))?;
        let response = self.recv()?;
        let coord = response.as_coordinate().expect("malformed server response");
        Ok(coord)
    }

    /// Returns the coordinate location of the block the player is standing on
    /// (i.e. tile)
    pub fn get_player_tile_position(&mut self) -> Result<Coordinate> {
        let mut coord = self.get_player_position()?;
        coord.y -= 1;
        Ok(coord)
    }

    /// Sets block at [`Coordinate`] to specified [`Block`]
    pub fn set_block(&mut self, location: impl Into<Coordinate>, block: Block) -> Result<()> {
        self.send(
            Command::new("world.setBlock")
                .arg_coordinate(location.into())
                .arg_block(block),
        )
    }

    /// Returns [`Block`] object from specified [`Coordinate`]
    pub fn get_block(&mut self, location: impl Into<Coordinate>) -> Result<Block> {
        self.send(Command::new("world.getBlockWithData").arg_coordinate(location.into()))?;
        let response = self.recv()?;
        let block = response.as_block().expect("malformed server response");
        Ok(block)
    }

    /// Sets a cuboid of blocks to all be the specified [`Block`], with the
    /// corners of the cuboid specified by [`Coordinate`]s `a` and `b` (in any
    /// order)
    pub fn set_blocks(
        &mut self,
        a: impl Into<Coordinate>,
        b: impl Into<Coordinate>,
        block: Block,
    ) -> Result<()> {
        self.send(
            Command::new("world.setBlocks")
                .arg_coordinate(a.into())
                .arg_coordinate(b.into())
                .arg_block(block),
        )
    }

    /// Returns a 3D `Vec` of the [`Block`]s of cuboid specified by
    ///  [`Coordinate`]s `a` and `b` (in any order)
    pub fn get_blocks(
        &mut self,
        a: impl Into<Coordinate>,
        b: impl Into<Coordinate>,
    ) -> Result<Chunk> {
        let a = a.into();
        let b = b.into();
        self.send(
            Command::new("world.getBlocksWithData")
                .arg_coordinate(a)
                .arg_coordinate(b),
        )?;
        let response = self.recv()?;
        let list = response.as_block_list().expect("malformed server response");
        let chunk = Chunk::new(a, b, list);
        Ok(chunk)
    }

    /// Returns the `y`-value of the highest solid block at the specified `x`
    /// and `z` coordinate
    ///
    /// **DO NOT USE FOR LARGE AREAS, IT WILL BE VERY SLOW**
    ///
    /// Use [`get_heights`] instead
    ///
    /// [`get_heights`]: Connection::get_heights
    pub fn get_height(&mut self, x: i32, z: i32) -> Result<i32> {
        self.send(Command::new("world.getHeight").arg_int(x).arg_int(z))?;
        let response = self.recv()?;
        let height = response.as_integer().expect("malformed server response");
        Ok(height)
    }

    /// Provides a scaled option of the [`get_height`] call to allow for considerable
    /// performance gains
    ///
    /// [`get_height`]: Connection::get_height
    pub fn get_heights(
        &mut self,
        a: impl Into<Coordinate>,
        b: impl Into<Coordinate>,
    ) -> Result<HeightMap> {
        let a = a.into();
        let b = b.into();
        self.send(
            Command::new("world.getHeights")
                .arg_int(a.x)
                .arg_int(a.z)
                .arg_int(b.x)
                .arg_int(b.z),
        )?;
        let response = self.recv()?;
        let list = response.as_integer_list();
        let height_map = HeightMap::new(a, b, list);
        Ok(height_map)
    }
}
