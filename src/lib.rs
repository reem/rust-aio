#![feature(phase, unboxed_closures, globs)]
#![deny(warnings)]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[phase(plugin, link)]
extern crate log;

extern crate event;
extern crate mio;
extern crate emitter;
extern crate nix;

pub use std::path;

pub use path::{
    Path
};

pub use error::{
    AioError,
    AioResult
};

pub mod error;
pub mod sys;

