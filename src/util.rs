use future::{AioFuture, Future};
use error::Kind;
use {IoReadStream, IoWriteStream};

pub struct MemReader {
    data: Vec<u8>,
    offset: usize
}

impl MemReader {
    pub fn new(vec: Vec<u8>) -> MemReader {
        MemReader {
            data: vec,
            offset: 0
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.data
    }

    pub fn as_bytes(&self) -> &[u8] {
        if self.offset >= self.data.len() {
            &[]
        } else {
            &self.data[self.offset..]
        }
    }

    pub fn done(&self) -> bool { self.data.len() <= self.offset }
}

impl IoReadStream for MemReader {
    type PipeResult = MemReader;

    fn pipe<W>(self, write: W) -> AioFuture<MemReader>
    where W: IoWriteStream {
        let (producer, consumer) = Future::pair();
        let mut producer = Some(producer);
        let mut this = Some(self);

        write.on_write(move |writer| {
            if loop {
                if this.as_ref().unwrap().done() { return false }

                match writer(this.as_ref().unwrap().as_bytes()) {
                    Ok(n) => { this.as_mut().unwrap().offset += n; },

                    Err(err) => {
                        return match err.kind {
                            Kind::Eof => false,
                            Kind::WouldBlock => true,
                            _ => {
                                producer.take().unwrap().fail(err);
                                false
                            }
                        };
                    }
                }
            } {
                producer.take().unwrap().complete(this.take().unwrap());
                true
            } else {
                false
            }
        });

        consumer
    }
}

