use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
    str::Split,
};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    const DEFAULT_ADDRESS: &'static str = "127.0.0.1:4711";

    pub fn new() -> io::Result<Self> {
        Self::with_address::<&str>(Self::DEFAULT_ADDRESS)
    }

    pub fn with_address<A>(addr: impl ToSocketAddrs) -> io::Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    fn send(&mut self, data: impl AsRef<[u8]>) -> io::Result<()> {
        self.stream.write(data.as_ref())?;
        Ok(())
    }

    fn recv(&mut self) -> io::Result<String> {
        let mut reader = BufReader::new(&self.stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;
        Ok(buffer)
    }

    pub fn post_to_chat(&mut self, message: impl AsRef<str>) -> io::Result<()> {
        self.send(format!("chat.post({})\n", sanitize_string(message)))
    }

    pub fn do_command(&mut self, command: impl AsRef<str>) -> io::Result<()> {
        self.send(format!("player.doCommand({})\n", sanitize_string(command)))
    }

    pub fn set_player_position(&mut self, pos: Coordinate) -> io::Result<()> {
        self.send(format!("player.setPos({},{},{})\n", pos.x, pos.y, pos.z))
    }

    pub fn get_player_position(&mut self) -> io::Result<Coordinate> {
        self.send("player.getPos()\n")?;
        let response = self.recv()?;
        let coord = coordinate_from_response(&response).expect("malformed server response");
        Ok(coord)
    }

    pub fn set_player_tile_position(&mut self, pos: Coordinate) -> io::Result<()> {
        self.send(format!(
            "player.setPos({},{},{})\n",
            pos.x,
            pos.y + 1,
            pos.z,
        ))
    }

    pub fn get_player_tile_position(&mut self) -> io::Result<Coordinate> {
        self.send("player.getPos()\n")?;
        let response = self.recv()?;
        let coord = coordinate_from_response(&response).expect("malformed server response");
        Ok(coord)
    }

    pub fn set_block(&mut self, location: Coordinate, block: Block) -> io::Result<()> {
        self.send(format!(
            "world.setBlock({},{},{},{},{})\n",
            location.x, location.y, location.z, block.id, block.modifier,
        ))
    }

    pub fn get_block(&mut self, location: Coordinate) -> io::Result<Block> {
        self.send(format!(
            "world.getBlockWithData({},{},{})\n",
            location.x, location.y, location.z,
        ))?;
        let response = self.recv()?;
        let block = block_from_response(&response).expect("malformed server response");
        Ok(block)
    }
}

fn sanitize_string(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    for ch in input.as_ref().chars() {
        match ch {
            '\n' => output.push(' '),
            '\t' | '\x20'..='\x7e' => output.push(ch),
            _ => (),
        }
    }
    output
}

fn coordinate_from_response(line: &str) -> Option<Coordinate> {
    let mut iter = IntegerList::from(&line);
    let x = iter.next()?;
    let y = iter.next()?;
    let z = iter.next()?;
    Some(Coordinate { x, y, z })
}

fn block_from_response(line: &str) -> Option<Block> {
    let mut iter = IntegerList::from(&line);
    let id = iter.next()?;
    let modifier = iter.next()?;
    Some(Block { id, modifier })
}

struct IntegerList<'a> {
    inner: Split<'a, char>,
}

impl<'a> IntegerList<'a> {
    pub fn from(line: &'a str) -> Self {
        Self {
            inner: line.split(','),
        }
    }
}

impl Iterator for IntegerList<'_> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.trim().parse::<f32>().ok()?.floor() as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coordinate {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: i32,
    pub modifier: i32,
}

impl Block {
    pub const fn new(id: i32, modifier: i32) -> Self {
        Self { id, modifier }
    }
}

macro_rules! blocks {
    ( $( $name:ident = ($id:expr, $modifier:expr); )* ) => {
        impl Block {
            $( pub const $name: Self = Self::new($id, $modifier); )*
        }
    };
}

blocks! {
    AIR = (0, 0);
    STONE = (1, 0);
    GRANITE = (1, 1);
    // TODO
}
