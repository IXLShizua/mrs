use std::io;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct ByteVec {
    inner: Vec<u8>,
}

impl ByteVec {
    pub fn new() -> ByteVec {
        Self::default()
    }
}

impl From<Vec<u8>> for ByteVec {
    fn from(value: Vec<u8>) -> Self {
        ByteVec { inner: value }
    }
}

impl From<ByteVec> for Vec<u8> {
    fn from(value: ByteVec) -> Self {
        value.inner
    }
}

impl io::Read for ByteVec {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut chunk = self
            .inner
            .get(0..buf.len())
            .ok_or(io::ErrorKind::UnexpectedEof)?;

        let read = chunk.read(buf)?;
        self.inner.drain(0..read);

        Ok(read)
    }
}

impl io::Write for ByteVec {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Deref for ByteVec {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ByteVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
