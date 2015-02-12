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
    fn pipe<W>(mut self, write: W) where W: IoWriteStream {
        write.on_write(move |writer| {
            loop {
                if self.done() { return false }

                match writer(self.as_bytes()) {
                    Ok(n) => { self.offset += n; },

                    Err(err) => {
                        return match err.kind {
                            Kind::Eof => false,
                            Kind::WouldBlock => true,
                            _ => false
                        };
                    }
                }
            }
        })
    }
}

