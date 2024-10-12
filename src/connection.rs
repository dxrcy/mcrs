use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{command::Command, Arg};
use crate::{response::Response, Block, Coordinate};

type Result<T> = io::Result<T>;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    const DEFAULT_ADDRESS: &'static str = "127.0.0.1:4711";

    pub fn new() -> Result<Self> {
        Self::with_address::<&str>(Self::DEFAULT_ADDRESS)
    }

    pub fn with_address<A>(addr: impl ToSocketAddrs) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    fn send(&mut self, command: Command) -> Result<()> {
        self.stream.write(command.build().as_bytes())?;
        Ok(())
    }

    fn recv(&mut self) -> Result<Response> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(Response::new(buffer))
    }

    pub fn post_to_chat(&mut self, message: impl Arg) -> Result<()> {
        self.send(Command::new("chat.post").arg(message))
    }

    pub fn do_command(&mut self, command: impl Arg) -> Result<()> {
        self.send(Command::new("player.doCommand").arg(command))
    }

    pub fn set_player_position(&mut self, position: Coordinate) -> Result<()> {
        self.send(
            Command::new("player.setPos")
                .arg(position.x)
                .arg(position.y)
                .arg(position.z),
        )
    }

    pub fn get_player_position(&mut self) -> Result<Coordinate> {
        self.send(Command::new("player.getPos"))?;
        let response = self.recv()?;
        let coord = response.as_coordinate().expect("malformed server response");
        Ok(coord)
    }

    pub fn set_player_tile_position(&mut self, position: Coordinate) -> Result<()> {
        self.send(
            Command::new("player.setPos")
                .arg(position.x)
                .arg(position.y + 1)
                .arg(position.z),
        )
    }

    pub fn get_player_tile_position(&mut self) -> Result<Coordinate> {
        self.send(Command::new("player.getPos"))?;
        let response = self.recv()?;
        let coord = response.as_coordinate().expect("malformed server response");
        Ok(coord)
    }

    pub fn set_block(&mut self, location: Coordinate, block: Block) -> Result<()> {
        self.send(
            Command::new("world.setBlock")
                .arg(location.x)
                .arg(location.y + 1)
                .arg(location.z)
                .arg(block.id)
                .arg(block.modifier),
        )
    }

    pub fn get_block(&mut self, location: Coordinate) -> Result<Block> {
        self.send(
            Command::new("world.getBlockWithData")
                .arg(location.x)
                .arg(location.y + 1)
                .arg(location.z),
        )?;
        let response = self.recv()?;
        let block = response.as_block().expect("malformed server response");
        Ok(block)
    }

    // TODO(feat): set_blocks
    // TODO(feat): get_blocks

    pub fn get_height(&mut self, x: i32, z: i32) -> Result<i32> {
        self.send(Command::new("world.getHeight").arg(x).arg(z))?;
        let response = self.recv()?;
        let height = response.as_integer().expect("malformed server response");
        Ok(height)
    }

    // TODO(feat): get_heights
}
