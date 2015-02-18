use {IoHandle, IoDesc};

pub use mio::{ReadHint, Interest, PollOpt};
pub use sys::register::EventHandler;
pub use event::{EventError, EventResult};

pub trait Register {
    type Error;

    fn register(self) -> Result<(), Self::Error>;
}

pub struct Registration<I, S, R, W> {
    io: I,
    shared: S,
    read: R,
    write: W,
    opts: (Interest, PollOpt)
}

impl<I, S, R, W> Registration<I, S, R, W>
where I: IoHandle + 'static,
      S: 'static,
      R: FnMut(&mut I, &mut S, ReadHint) -> bool + 'static,
      W: FnMut(&mut I, &mut S) -> bool + 'static {
   #[inline]
   pub fn new(io: I, shared: S, read: R, write: W) -> Registration<I, S, R, W> {
        Registration {
            io: io,
            shared: shared,
            read: read,
            write: write,
            opts: (Interest::readable(), PollOpt::level())
        }
   }

   #[inline]
   pub fn with_opts(io: I, shared: S, read: R, write: W,
                    interest: Interest, opt: PollOpt) -> Registration<I, S, R, W> {
        Registration {
            io: io,
            shared: shared,
            read: read,
            write: write,
            opts: (interest, opt)
        }
   }
}

impl<I, S, R, W> Registration<I, S, R, W> {
   #[inline]
   pub fn io(&self) -> &I { &self.io }
}

impl<I: IoHandle, S, R, W> IoHandle for Registration<I, S, R, W> {
    fn desc(&self) -> &IoDesc { self.io().desc() }
}

impl<I, S, R, W> Register for Registration<I, S, R, W>
where I: IoHandle + 'static,
      S: 'static,
      R: FnMut(&mut I, &mut S, ReadHint) -> bool + 'static,
      W: FnMut(&mut I, &mut S) -> bool + 'static {
    type Error = EventError;

    fn register(self) -> EventResult<()> {
        use sys::register;
        register(self)
    }
}

impl<I, S, R, W> Evented for Registration<I, S, R, W>
where S: 'static,
      R: FnMut(&mut I, &mut S, ReadHint) -> bool + 'static,
      W: FnMut(&mut I, &mut S) -> bool + 'static {
   #[inline]
    fn readable(&mut self, hint: ReadHint) -> bool {
        (self.read)(&mut self.io, &mut self.shared, hint)
    }

   #[inline]
    fn writable(&mut self) -> bool {
        (self.write)(&mut self.io, &mut self.shared)
    }
}


impl<I, S, R, W> Configured for Registration<I, S, R, W> {
    #[inline]
    fn cfg(&self) -> (Interest, PollOpt) {
        self.opts
    }
}

pub trait Evented {
    fn readable(&mut self, _: ReadHint) -> bool { true }
    fn writable(&mut self) -> bool { true }
}

pub trait Configured {
   #[inline]
    fn cfg(&self) -> (Interest, PollOpt) {
        (Interest::readable(), PollOpt::level())
    }
}

