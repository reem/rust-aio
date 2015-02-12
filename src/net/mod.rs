use iostream::IoReadStream;
use eventstream::EventStream;

use error::AioError;

pub use sys::socket::{SocketAddr, IpAddr};

pub trait IoAcceptor {
    type Stream: IoReadStream;

    fn accept(self) -> EventStream<Self::Stream, AioError>;
}

pub mod tcp;
//pub mod udp;

