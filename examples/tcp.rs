extern crate aio;

use aio::net::SocketAddress;
use aio::net::tcp::{TcpAcceptor, TcpSocket};

const RESPONSE: &'static str = "HTTP/1.1 200 OK\r
Content-Length: 14\r
\r
Hello World\r
\r";

fn main() {
    TcpSocket::new().unwrap()
        .bind(&"localhost:3000".parse::<SocketAddress>().unwrap()).unwrap()
        .listen().unwrap()
        .map(|sock| { RESPONSE.pipe(sock); });
}

