use sys::{IoDesc};
use {AioResult};

pub use mio::net::{AddressFamily, SocketType};
pub use mio::net::SockAddr as SocketAddress;

use mio::os;

pub fn socket(af: AddressFamily, st: SocketType) -> AioResult<IoDesc> {
    Ok(try!(os::socket(af, st)))
}

pub fn connect(io: &IoDesc, addr: &SocketAddress) -> AioResult<bool> {
    Ok(try!(os::connect(io, addr)))
}

pub fn bind(io: &IoDesc, addr: &SocketAddress) -> AioResult<()> {
    Ok(try!(os::bind(io, addr)))
}

pub fn listen(io: &IoDesc, backlog: uint) -> AioResult<()> {
    Ok(try!(os::listen(io, backlog)))
}

pub fn accept(io: &IoDesc) -> AioResult<IoDesc> {
    Ok(try!(os::accept(io)))
}

pub fn recvfrom(io: &IoDesc, buf: &mut [u8]) -> AioResult<(uint, SocketAddress)> {
    Ok(try!(os::recvfrom(io, buf)))
}

pub fn sendto(io: &IoDesc, buf: &[u8], tgt: &SocketAddress) -> AioResult<uint> {
    Ok(try!(os::sendto(io, buf, tgt)))
}

