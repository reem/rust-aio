//! Streams of discrete events.

pub use syncbox::util::async::Stream as EventStream;
pub use syncbox::util::async::{StreamIter, Sender};

use {AioError};

pub type AioEventStream<T> = EventStream<T, AioError>;

