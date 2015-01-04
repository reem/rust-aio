use event::self;
use {IoHandle, IoDesc};

pub use mio::event::{ReadHint, Interest, PollOpt};

pub trait Io: IoHandle {
    fn readable(&mut self, hint: ReadHint) -> bool { }
    fn writable(&mut self) -> bool { }

    fn cfg(&self) -> (Interest, PollOpt) {
        (Interest::READABLE, PollOpt::LEVEL)
    }

    fn register(self) {
        event::register(self);
    }
}

impl<I: Io> event::Handler for I {
    fn readable(&mut self, hint: evt::ReadHint) -> bool { self.readable(hint) }
    fn writable(&mut self) -> bool { self.writable() }

    fn desc(&self) -> &IoDesc { self.desc() }
    fn interest(&self) -> Option<Interest> { Some(self.cfg().0) }
    fn opt(&self) -> Option<PollOpt> { Some(self.cfg().1) }
}

