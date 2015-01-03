use sys::{IoDesc};
use {AioResult};

pub use mio::net::{AddressFamily, SocketType};
pub use mio::net::SockAddr as SocketAddress;
pub use std::io::net::ip::IpAddr;

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


pub fn set_reuseaddr(io: &IoDesc, val: bool) -> AioResult<()> {
    Ok(try!(os::set_reuseaddr(io, val)))
}

pub fn set_reuseport(io: &IoDesc, val: bool) -> AioResult<()> {
    Ok(try!(os::set_reuseport(io, val)))
}

pub fn set_tcp_nodelay(io: &IoDesc, val: bool) -> AioResult<()> {
    Ok(try!(os::set_tcp_nodelay(io, val)))
}

pub fn join_multicast_group(io: &IoDesc, addr: &IpAddr,
                            interface: &Option<IpAddr>) -> AioResult<()> {
    Ok(try!(os::join_multicast_group(io, addr, interface)))

}

pub fn leave_multicast_group(io: &IoDesc, addr: &IpAddr,
                             interface: &Option<IpAddr>) -> AioResult<()> {
    Ok(try!(os::leave_multicast_group(io, addr, interface)))

}

pub fn set_multicast_ttl(io: &IoDesc, val: u8) -> AioResult<()> {
    Ok(try!(os::set_multicast_ttl(io, val)))
}

pub fn linger(io: &IoDesc) -> AioResult<uint> {
    Ok(try!(os::linger(io)))
}

pub fn set_linger(io: &IoDesc, dur_s: uint) -> AioResult<()> {
    Ok(try!(os::set_linger(io, dur_s)))
}

