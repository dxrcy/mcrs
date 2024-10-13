use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{command::Command, Chunk};
use crate::{response::Response, Block, Coordinate};

type Result<T> = io::Result<T>;

#[derive(Debug)]
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
        self.stream.write_all(command.build().as_bytes())?;
        Ok(())
    }

    fn recv(&mut self) -> Result<Response> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(Response::new(buffer))
    }

    pub fn post_to_chat(&mut self, message: impl AsRef<str>) -> Result<()> {
        self.send(Command::new("chat.post").arg_string(message))
    }

    pub fn do_command(&mut self, command: impl AsRef<str>) -> Result<()> {
        self.send(Command::new("player.doCommand").arg_string(command))
    }

    pub fn set_player_position(&mut self, position: Coordinate) -> Result<()> {
        self.send(Command::new("player.setPos").arg_coordinate(position))
    }

    pub fn set_player_tile_position(&mut self, mut position: Coordinate) -> Result<()> {
        position.y += 1;
        self.set_player_position(position)
    }

    pub fn get_player_position(&mut self) -> Result<Coordinate> {
        self.send(Command::new("player.getPos"))?;
        let response = self.recv()?;
        let coord = response.as_coordinate().expect("malformed server response");
        Ok(coord)
    }

    pub fn get_player_tile_position(&mut self) -> Result<Coordinate> {
        let mut coord = self.get_player_position()?;
        coord.y -= 1;
        Ok(coord)
    }

    pub fn set_block(&mut self, location: Coordinate, block: Block) -> Result<()> {
        self.send(
            Command::new("world.setBlock")
                .arg_coordinate(location)
                .arg_block(block),
        )
    }

    pub fn get_block(&mut self, location: Coordinate) -> Result<Block> {
        self.send(Command::new("world.getBlockWithData").arg_coordinate(location))?;
        let response = self.recv()?;
        let block = response.as_block().expect("malformed server response");
        Ok(block)
    }

    // TODO(feat): set_blocks

    pub fn get_blocks(&mut self, location_a: Coordinate, location_b: Coordinate) -> Result<Chunk> {
        self.send(
            Command::new("world.getBlocksWithData")
                .arg_coordinate(location_a)
                .arg_coordinate(location_b),
        )?;
        let response = self.recv()?;
        println!("{:?}", response);
        let list = response.as_block_list().expect("malformed server response");
        println!("{:?}", list.len());
        println!("{:?}", list);
        for block in &list {
            println!("{}", block);
        }
        let chunk = Chunk::new(location_a, location_b, list);
        Ok(chunk)
    }

    pub fn get_height(&mut self, x: i32, z: i32) -> Result<i32> {
        self.send(Command::new("world.getHeight").arg_int(x).arg_int(z))?;
        let response = self.recv()?;
        let height = response.as_integer().expect("malformed server response");
        Ok(height)
    }

    // TODO(feat): get_heights
}
