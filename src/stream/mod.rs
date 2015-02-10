use register::{Registration, EventHandler, Register};
use constants::BUFFER_SIZE;

use buf::{RingBuf, Buf, MutBuf};

use sys::rcmut::RcMut;
use sys::{IoRead, IoWrite};

use self::AsyncActionResult::{Done, MoreLater};

use {AioResult, error};

pub trait IoReadStream {
    fn pipe<W>(self, write: W) where W: IoWriteStream;
}

pub trait IoWriteStream {
    type Writer: FnMut(&[u8]) -> AioResult<usize>;

    fn on_write<W: FnMut(&mut Self::Writer) -> bool + 'static>(self, W);
}

pub trait IoDuplexStream: IoReadStream + IoWriteStream {

}

// Unique stream over an IO
pub struct IoReader<I> {
    io: I
}

impl<I: EventHandler + IoRead> IoReadStream for IoReader<I> {
    fn pipe<W>(self, write: W) where W: IoWriteStream {
        let backbuffer = RingBuf::new(BUFFER_SIZE);

        let mut read_handle = RcMut::new(backbuffer);
        let mut write_handle = read_handle.clone();

        let on_read = move |io: &mut I, _| {
            into_continue(read_to(io, unsafe { read_handle.borrow_mut() }))
        };

        let on_write = move |writer: &mut <W as IoWriteStream>::Writer| {
            into_continue(write_from(unsafe { write_handle.borrow_mut() }, writer))
        };

        Registration::new(self.io, on_read, |_: &mut I| incorrect_writable()).register();
        write.on_write(on_write);
    }
}

// Unique writer to an IO
pub struct IoWriter<I> {
    io: I
}

impl<I: EventHandler + IoWrite> IoWriteStream for IoWriter<I> {
    type Writer = WriterTo<I>;

    fn on_write<W: FnMut(&mut WriterTo<I>) -> bool + 'static>(self, mut listener: W) {
        let on_write = move |io: &mut I| {
            let mut writer = WriterTo { io: io };
            listener(&mut writer)
        };

        Registration::new(self.io, |_: &mut I, _| incorrect_readable(), on_write).register();
    }
}

struct WriterTo<I> {
    // Lies not 'static actually, but limited by the usage site.
    io: *mut I
}

impl<'a, I: IoWrite> FnMut<(&'a [u8],)> for WriterTo<I> {
    type Output = AioResult<usize>;

    extern "rust-call" fn call_mut(&mut self, (bytes,): (&[u8],)) -> AioResult<usize> {
        unsafe { &mut *self.io }.write(bytes)
    }
}

fn read_to<I: IoRead>(io: &mut I, buffer: &mut RingBuf) -> AioResult<AsyncActionResult> {
    let mut writer = buffer.writer();
    if !writer.has_remaining() { return Ok(MoreLater) }

    loop {
        match io.read(writer.mut_bytes()) {
            Ok(n) => {
                writer.advance(n);
                if !writer.has_remaining() { return Ok(MoreLater) }
            },
            Err(err) => {
                return match err.kind {
                    error::Kind::Eof => Ok(Done),
                    error::Kind::WouldBlock => Ok(MoreLater),
                    _ => Err(err)
                };
            }
        }
    }
}

fn write_from<I: FnMut(&[u8]) -> AioResult<usize>>(from: &mut RingBuf, to: &mut I) -> AioResult<AsyncActionResult> {
    let mut reader = from.reader();

    if !reader.has_remaining() { return Ok(MoreLater) }

    loop {
        match to(reader.bytes()) {
            Ok(n) => {
                reader.advance(n);
                if !reader.has_remaining() { return Ok(MoreLater) }
            },

            Err(err) => {
                return match err.kind {
                    error::Kind::Eof => Ok(Done),
                    error::Kind::WouldBlock => Ok(MoreLater),
                    _ => Err(err)
                };
            }
        }
    }
}

enum AsyncActionResult { Done, MoreLater }

fn into_continue(res: AioResult<AsyncActionResult>) -> bool {
    match res {
        Ok(Done) => false,
        Ok(MoreLater) => true,
        Err(_) => false
    }
}

fn incorrect_writable() -> ! {
    panic!("Received writable on a readable registration")
}

fn incorrect_readable() -> ! {
    panic!("Received readable on a writable registration")
}



