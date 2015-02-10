#![feature(unboxed_closures, core, io, hash, os)]
#![cfg_attr(test, deny(warnings))]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;

extern crate event;
extern crate mio;

pub use std::path;

pub use path::{
    Path
};

pub use error::{
    AioError,
    AioResult,
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
pub mod iostream;
pub mod buf {
    pub use mio::buf::{Buf, MutBuf, RingBuf,
                       RingBufReader, RingBufWriter};
}

mod register;

