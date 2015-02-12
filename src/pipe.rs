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
    pipe: Io<pipe::PipeReader>
}

impl PipeReader {
    pub fn from_raw(pipe: pipe::PipeReader) -> PipeReader {
        PipeReader { pipe: Io::new(pipe) }
    }
}

pub struct PipeWriter {
    pipe: Io<pipe::PipeWriter>
}

impl PipeWriter {
    pub fn from_raw(pipe: pipe::PipeWriter) -> PipeWriter {
        PipeWriter { pipe: Io::new(pipe) }
    }
}

impl IoReadStream for PipeReader {
    fn pipe<W>(self, write: W) where W: IoWriteStream {
        self.pipe.pipe(write)
    }
}

impl IoWriteStream for PipeWriter {
    type Writer = <Io<pipe::PipeWriter> as IoWriteStream>::Writer;

    fn on_write<W>(self, listener: W)
    where W: FnMut(&mut <PipeWriter as IoWriteStream>::Writer) -> bool + 'static {
        self.pipe.on_write(listener)
    }
}

