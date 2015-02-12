#![feature(unboxed_closures, core, io, hash, os)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, feature(test))]

//! Blazingly fast non-blocking IO.

#[cfg(test)]
extern crate test;

#[macro_use]
extern crate log;

extern crate event;
extern crate mio;
extern crate syncbox;

pub use iostream::{
    IoReadStream,
    IoWriteStream
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
pub mod eventstream;
pub mod net;
pub mod util;
pub mod buf {
    pub use mio::buf::{Buf, MutBuf, RingBuf,
                       RingBufReader, RingBufWriter};
}

mod register;
mod internals;
mod impls;

