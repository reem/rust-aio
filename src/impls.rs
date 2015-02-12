use util::MemReader;
use {IoReadStream, IoWriteStream};

impl IoReadStream for String {
    fn pipe<W>(self, write: W) where W: IoWriteStream {
        self.into_bytes().pipe(write)
    }
}

impl IoReadStream for Vec<u8> {
    fn pipe<W>(self, write: W) where W: IoWriteStream {
        MemReader::new(self).pipe(write)
    }
}

