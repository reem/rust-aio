extern crate event;
extern crate aio;
extern crate mio;

#[macro_use]
extern crate env_logger;

use aio::net::{SocketAddr, IoAcceptor};
use aio::net::tcp::TcpSocket;

use aio::{IoReadStream};

const RESPONSE: &'static str =
"HTTP/1.1 200 OK\r
Content-Length: 14\r
\r
Hello World\r
\r";

fn main() {
    env_logger::init().unwrap();

    event::next(|| {
        TcpSocket::v4().unwrap()
            .bind(&SocketAddr::parse("127.0.0.1:3000").unwrap()).unwrap()
            .listen(256).unwrap()
            .accept().map(|sock| RESPONSE.to_string().pipe(sock));
    }).unwrap();
    event::run().unwrap();
}

