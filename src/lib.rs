#![cfg_attr(test, deny(warnings))]
// #![deny(missing_docs)]

//! # aio
//!
//!

extern crate mio;

use std::io;

use mio::{EventLoop, Token, EventSet};
use mio::util::Slab;

pub trait EventMachine<S: Source<Self, Event=Self::Event>> {
    type Event;

    fn advance(&mut self, source: &mut S, event: Self::Event);
}

pub trait Source<E: EventMachine<Self, Event=Self::Event>> {
    type Event;
    type Error;

    fn feed(&mut self, machine: E) -> Result<(), Self::Error>;
}
