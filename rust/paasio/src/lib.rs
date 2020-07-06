use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    stream: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            stream: wrapped,
            reads: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.stream
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.stream.read(buf);
        match result {
            Ok(size) => {
                self.reads += 1;
                self.bytes_through += size;
            }
            _ => (),
        }
        result
    }
}

pub struct WriteStats<W> {
    stream: W,
    writes: usize,
    bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            stream: wrapped,
            writes: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.stream
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.stream.write(buf);

        match result {
            Ok(size) => {
                self.bytes_through += size;
                self.writes += 1;
            }
            _ => (),
        }
        result
    }

    fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }
}
