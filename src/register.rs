use {IoHandle, IoDesc};

pub use mio::event::{ReadHint, Interest, PollOpt};

pub trait Register {
    fn register(self);
}

pub struct Registration<I, R, W> {
    io: I,
    read: R,
    write: W,
    opts: (Interest, PollOpt)
}

impl<I, R, W> Registration<I, R, W> {
   #[inline]
   pub fn new(io: I, read: R, write: W) -> Registration<I, R, W> {
        Registration {
            io: io,
            read: read,
            write: write,
            opts: (Interest::readable(), PollOpt::level())
        }
   }

   #[inline]
   pub fn with_opts(io: I, read: R, write: W,
                    interest: Interest,
                    opt: PollOpt) -> Registration<I, R, W> {
        Registration {
            io: io,
            read: read,
            write: write,
            opts: (interest, opt)
        }
   }

   #[inline]
   pub fn io(&self) -> &I { &self.io }
}

impl<I: IoHandle, R, W> IoHandle for Registration<I, R, W> {
    fn desc(&self) -> &IoDesc { self.io().desc() }
}

impl<I, R, W> Register for Registration<I, R, W>
where I: IoHandle + 'static,
      R: FnMut(&mut I, ReadHint) -> bool + 'static,
      W: FnMut(&mut I) -> bool + 'static {
    fn register(self) {
        use sys::register;
        register(self)
    }
}

impl<I, R, W> Evented for Registration<I, R, W>
where R: FnMut(&mut I, ReadHint) -> bool,
      W: FnMut(&mut I) -> bool {
   #[inline]
    fn readable(&mut self, hint: ReadHint) -> bool {
        (self.read)(&mut self.io, hint)
    }

   #[inline]
    fn writable(&mut self) -> bool {
        (self.write)(&mut self.io)
    }
}


impl<I, R, W> Configured for Registration<I, R, W> {
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

