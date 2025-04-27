use std::io;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Stream {
    inner: TcpStream,
}

impl Stream {
    /// Default server address and port for [ELCI].
    ///
    /// [ELCI]: https://github.com/rozukke/elci
    pub const DEFAULT_ADDRESS: &'static str = "127.0.0.1:4711";

    pub fn new() -> io::Result<Self> {
        Ok(Self {
            inner: TcpStream::connect(Self::DEFAULT_ADDRESS)?,
        })
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            inner: self.inner.try_clone()?,
        })
    }
}

impl io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
