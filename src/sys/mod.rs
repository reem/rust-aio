pub use mio::IoDesc;

pub use self::register::{
    register
};

use mio::IoHandle as MioHandle;
use AioResult;

pub trait Read {
    fn read(&mut self, buf: &[u8]) -> AioResult<uint>;
}

pub trait Write {
    fn write(&mut self, buf: &mut [u8]) -> AioResult<uint>;
}

// Replacement trait to allow implementations with IoHandle
// in this crate.
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

