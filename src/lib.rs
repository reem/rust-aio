#![feature(phase, unboxed_closures, globs)]
#![deny(warnings)]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[phase(plugin, link)]
extern crate log;

extern crate event;
extern crate mio;
extern crate nix;

pub use std::path;

pub use path::{
    Path
};

pub use error::{
    AioError,
    AioResult
};

pub use sys::{
    IoHandle,
    IoDesc
};

pub use register::{
    Evented,
    Configured
};

pub mod error;
pub mod sys;
pub mod constants;

mod register;

