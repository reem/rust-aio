use {IoHandle, IoDesc, Evented, Configured};
use register::{ReadHint, Interest, PollOpt};

pub trait EventHandler: IoHandle + Evented + Configured + 'static {}
impl<I: IoHandle + Evented + Configured + 'static> EventHandler for I {}

struct AsHandler<T>(T);

impl<I: EventHandler> ::event::Handler for AsHandler<I> {
   #[inline]
    fn readable(&mut self, hint: ReadHint) -> bool { self.0.readable(hint) }

   #[inline]
    fn writable(&mut self) -> bool { self.0.writable() }

   #[inline]
    fn desc(&self) -> &IoDesc { self.0.desc() }

   #[inline]
    fn interest(&self) -> Option<Interest> { Some(self.0.cfg().0) }

   #[inline]
    fn opt(&self) -> Option<PollOpt> { Some(self.0.cfg().1) }
}

pub fn register<E: EventHandler>(ev: E) {
    ::event::register(AsHandler(ev))
}

