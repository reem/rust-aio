#![feature(phase, unboxed_closures)]
#![deny(missing_docs, warnings)]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[phase(plugin, link)]
extern crate log;

extern crate event;
extern crate mio;
extern crate emitter;

pub use std::path;

pub use error::{
    AioError,
    AioResult
};

pub mod error;

