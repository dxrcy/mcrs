use std::io;
use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub struct Stream {
    inner: UnixStream,
}

impl Stream {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            inner: UnixStream::connect("/tmp/elci-proxy")?,
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
