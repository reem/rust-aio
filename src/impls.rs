use future::AioFuture;
use util::MemReader;
use {IoReadStream, IoWriteStream, AioResult};

impl IoReadStream for String {
    type PipeResult = String;

    fn pipe<W>(self, write: W) -> AioFuture<String>
    where W: IoWriteStream {
        self.into_bytes().pipe(write)
            .map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
    }
}

impl IoReadStream for Vec<u8> {
    type PipeResult = Vec<u8>;

    fn pipe<W>(self, write: W) -> AioFuture<Vec<u8>>
    where W: IoWriteStream {
        MemReader::new(self).pipe(write)
            .map(|stream| stream.into_inner())
    }
}

struct WriterToMem<'a>(&'a mut Vec<u8>);

impl<'a, 'b> FnMut<(&'b [u8],)> for WriterToMem<'a> {
    type Output = AioResult<usize>;

    extern "rust-call" fn call_mut(&mut self, (bytes,): (&[u8],)) -> AioResult<usize> {
        self.0.push_all(bytes);
        Ok(bytes.len())
    }
}

impl<'a> IoWriteStream for &'a mut Vec<u8> {
    type Writer = WriterToMem<'a>;

    fn on_write<W>(self, mut listener: W)
    where W: FnMut(&mut <&'a mut Vec<u8> as IoWriteStream>::Writer) -> bool + 'static {
        listener(&mut WriterToMem(self));
    }
}

