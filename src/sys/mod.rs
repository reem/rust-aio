pub use mio::IoDesc;

pub use self::register::{
    register
};

use mio::IoHandle as MioHandle;
use mio::IoReader as MioReader;
use mio::IoWriter as MioWriter;

use error::AioError;
use AioResult;

pub trait IoRead: IoHandle {
    fn read(&mut self, buf: &mut [u8]) -> AioResult<usize>;
}

impl<I: MioReader + IoHandle> IoRead for I {
    fn read(&mut self, buf: &mut [u8]) -> AioResult<usize> {
        AioError::from_nonblock(self.read_slice(buf))
    }
}

pub trait IoWrite: IoHandle {
    fn write(&mut self, buf: &[u8]) -> AioResult<usize>;
}

impl<I: MioWriter + IoHandle> IoWrite for I {
    fn write(&mut self, buf: &[u8]) -> AioResult<usize> {
        AioError::from_nonblock(self.write_slice(buf))
    }
}

// Replacement trait to allow certain implementations in this crate.
pub trait IoHandle {
    fn desc(&self) -> &IoDesc;
}

impl<I: MioHandle> IoHandle for I {
    fn desc(&self) -> &IoDesc {
        fn mio_desc<M: MioHandle>(m: &M) -> &IoDesc { m.desc() }
        mio_desc(self)
    }
}

pub mod socket;
pub mod register;
pub mod rcmut;
pub mod pipe;

