use register::{Registration, Register, ReadHint};
use constants::BUFFER_SIZE;
use buf::{RingBuf, Buf, MutBuf};
use future::{AioFuture, Future, Complete};

use sys::rcmut::RcMut;
use sys::{IoRead, IoWrite};

use self::AsyncActionResult::{Done, MoreLater};

use {AioResult, error};

pub trait IoReadStream {
    type PipeResult = Self;

    fn pipe<W>(self, write: W) -> AioFuture<Self::PipeResult>
    where W: IoWriteStream;
}

pub trait IoWriteStream {
    type Writer: FnMut(&[u8]) -> AioResult<usize>;

    fn on_write<W>(self, W)
    where W: FnMut(&mut Self::Writer) -> bool + 'static;
}

pub trait IoDuplexStream: IoReadStream + IoWriteStream {

}

// Unique stream over an IO
pub struct Io<I> {
    io: I
}

impl<I> Io<I> {
    pub fn new(io: I) -> Io<I> {
        Io {
            io: io
        }
    }
}

impl<I: IoRead + 'static> IoReadStream for Io<I> {
    type PipeResult = ();

    fn pipe<W>(self, write: W) -> AioFuture<()>
    where W: IoWriteStream {
        let (producer, consumer) = Future::pair();
        let buffer = RingBuf::new(BUFFER_SIZE);

        let on_read = move |io: &mut I, shared: &mut (RingBuf, Option<Complete<_, _>>), _| {
            let (ref mut backbuffer, ref mut producer) = *shared;

            match read_to(io, backbuffer) {
                Ok(Done) => {
                    producer.take().unwrap().complete(());
                    false
                },
                Ok(MoreLater) => true,
                Err(e) => {
                    producer.take().unwrap().fail(e);
                    return false;
                }
            }
        };

        let on_write = move |writer: &mut <W as IoWriteStream>::Writer,
                             shared: &mut (RingBuf, Option<Complete<_, _>>)| {
            let (ref mut backbuffer, ref mut producer) = *shared;

            match write_from(backbuffer, writer) {
                Ok(Done) => {
                    producer.take().unwrap().complete(());
                    false
                },
                Ok(MoreLater) => true,
                Err(e) => {
                    producer.take().unwrap().fail(e);
                    return false;
                }
            }
        };

        register_shared(
            self.io,
            (buffer, Some(producer)),
            on_read,
            move |_: &mut I, _| incorrect_writable(),
            write,
            on_write
        );

        consumer
    }
}

fn register_shared<I, S, R, W, Wr, WrC>(io: I, shared: S, mut read: R, mut write: W,
                                        writer: Wr, mut on_write: WrC)
where I: IoRead + 'static, S: 'static,
      R: FnMut(&mut I, &mut S, ReadHint) -> bool + 'static,
      W: FnMut(&mut I, &mut S) -> bool + 'static,
      Wr: IoWriteStream,
      WrC: FnMut(&mut <Wr as IoWriteStream>::Writer, &mut S) -> bool + 'static {

    let mut shared = RcMut::new(shared);

    Registration::new(
        io,
        shared.clone(),
        move |io, shr, hint| read(io, unsafe { shr.borrow_mut() }, hint),
        move |io, shr| write(io, unsafe { shr.borrow_mut() })
    // FIXME: unwrap
    ).register().unwrap();

    writer.on_write(move |writer| {
        on_write(writer, unsafe { shared.borrow_mut() })
    });
}

impl<I: IoWrite + 'static> IoWriteStream for Io<I> {
    type Writer = WriterTo<I>;

    fn on_write<W>(self, mut listener: W)
    where W: FnMut(&mut WriterTo<I>) -> bool + 'static {
        let on_write = move |io: &mut I, &mut (): &mut ()| {
            let mut writer = WriterTo { io: io };
            listener(&mut writer)
        };

        Registration::new(
            self.io,
            (),
            move |_: &mut I, &mut (), _| incorrect_readable(),
            on_write
        // FIXME: unwrap
        ).register().unwrap();
    }
}

struct WriterTo<I> {
    // Lies not 'static actually, but limited by the usage site.
    pub io: *mut I
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

    loop {
        if !reader.has_remaining() { return Ok(MoreLater) }

        match to(reader.bytes()) {
            Ok(n) => reader.advance(n),

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

fn incorrect_writable() -> ! {
    panic!("Received writable on a readable registration")
}

fn incorrect_readable() -> ! {
    panic!("Received readable on a writable registration")
}

#[cfg(test)]
mod test;

