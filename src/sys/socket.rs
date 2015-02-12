//! A facade over mio's networking facilities.

// General stuff
pub use mio::net::{
    // Types
    AddressFamily,
    SocketType,

    // Traits
    Socket,
    MulticastSocket,
    UnconnectedSocket
};

// Related Traits
pub use mio::{
    IoAcceptor
};

// Tcp utilities
pub use mio::net::tcp::{
    TcpSocket,
    TcpListener,
    TcpAcceptor
};

// Udp utilities
pub use mio::net::udp::{
    UdpSocket
};

// Aliases
pub use mio::net::SockAddr as SocketAddr;
pub use std::old_io::net::ip::IpAddr;

