use register::Io;
use constants::BUFFER_SIZE;
use {IoHandle};

pub trait Read {
    fn read(&mut self, buf: &[u8]) -> AioResult<uint>;
}

pub trait Write {
    fn write(&mut self, buf: &mut [u8]) -> AioResult<uint>;
}

pub struct Stream<I: Io> {
    io: I,
    buffer: Vec<u8>
}

// All Streams
impl<I: Io> Stream<I> {
    pub fn new(io: I) -> Stream<I> {
        Stream {
            io: I,
            buffer: Vec::with_capacity(BUFFER_SIZE)
        }
    }
}

// Read Streams
impl<I: Io + Read> Stream<I> {
    pub fn pipe<O, W>(self, write: W)
    where O: Io + Write, W: Stream<O> {

    }
}

// Write Streams
impl<I: Io + Write> Stream<I> {
    pub fn pipe_from<O, R>(self, read: R)
    where O: Io + Read, R: Stream<O> {

    }
}

// Duplex Streams
impl<I: Io + Read + Write> Stream<I> {
    pub fn transform<O, T>(self, transform: T) -> Transform<Self, T>
    where O: Io + Read + Write, T: Stream<O> {

    }
}

