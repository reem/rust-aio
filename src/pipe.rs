use future::AioFuture;
use iostream::{Io, IoReadStream, IoWriteStream};
use sys::pipe;

use std::error::FromError;

use {AioResult};

pub fn pipe() -> AioResult<(PipeReader, PipeWriter)> {
    pipe::pipe().map(|(rd, wr)| {
        (PipeReader::from_raw(rd), PipeWriter::from_raw(wr))
    }).map_err(FromError::from_error)
}

pub struct PipeReader {
    inner: Io<pipe::PipeReader>
}

impl PipeReader {
    pub fn from_raw(pipe: pipe::PipeReader) -> PipeReader {
        PipeReader { inner: Io::new(pipe) }
    }
}

pub struct PipeWriter {
    inner: Io<pipe::PipeWriter>
}

impl PipeWriter {
    pub fn from_raw(pipe: pipe::PipeWriter) -> PipeWriter {
        PipeWriter { inner: Io::new(pipe) }
    }
}

impl IoReadStream for PipeReader {
    type PipeResult = ();

    fn pipe<W>(self, write: W) -> AioFuture<()>
    where W: IoWriteStream {
        self.inner.pipe(write)
    }
}

impl IoWriteStream for PipeWriter {
    type Writer = <Io<pipe::PipeWriter> as IoWriteStream>::Writer;

    fn on_write<W>(self, listener: W)
    where W: FnMut(&mut <PipeWriter as IoWriteStream>::Writer) -> bool + 'static {
        self.inner.on_write(listener)
    }
}

#[cfg(test)]
mod tests {
    use super::pipe;
    use IoReadStream;

    #[test]
    fn test_pipe() {
        let (rd, wr) = pipe().unwrap();

        ::event::next(move || {
            let mut vec = vec![];
            "hello".to_string().pipe(wr);
            rd.pipe(&mut vec).map(move |()| {
                assert_eq!(&vec[], b"hello")
            });
        }).unwrap();
        ::event::run().unwrap();
    }
}

