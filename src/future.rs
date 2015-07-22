use AioError;

pub use syncbox::util::async::{Future, Complete};
pub type AioFuture<T> = Future<T, AioError>;

