use register::{Registration, Register, Interest, PollOpt};
use iostream::{IoReadStream, IoWriteStream, Io};
use eventstream::{EventStream, AioEventStream};
use future::AioFuture;
use net::{IoAcceptor, SocketAddr};
use sys::socket;

use sys::socket::IoAcceptor as MioAcceptor;

use std::error::FromError;

use {AioError, AioResult};

pub struct TcpSocket {
    socket: socket::TcpSocket
}

impl TcpSocket {
    pub fn from_raw(socket: socket::TcpSocket) -> TcpSocket {
        TcpSocket { socket: socket }
    }

    pub fn v4() -> AioResult<TcpSocket> {
        socket::TcpSocket::v4()
            .map(TcpSocket::from_raw).map_err(FromError::from_error)
    }

    pub fn v6() -> AioResult<TcpSocket> {
        socket::TcpSocket::v6()
            .map(TcpSocket::from_raw).map_err(FromError::from_error)
    }

    pub fn connect(&self, addr: &SocketAddr) -> AioResult<()> {
        self.socket.connect(addr).map_err(FromError::from_error)
    }

    pub fn bind(self, addr: &SocketAddr) -> AioResult<TcpListener> {
        self.socket.bind(addr)
            .map(TcpListener::from_raw).map_err(FromError::from_error)
    }

    pub fn peername(&self) -> AioResult<SocketAddr> {
        self.socket.getpeername().map_err(FromError::from_error)
    }

    pub fn sockname(&self) -> AioResult<SocketAddr> {
        self.socket.getsockname().map_err(FromError::from_error)
    }
}

pub struct TcpListener {
    listener: socket::TcpListener
}

impl TcpListener {
    pub fn from_raw(listener: socket::TcpListener) -> TcpListener {
        TcpListener { listener: listener }
    }

    pub fn listen(self, backlog: usize) -> AioResult<TcpAcceptor> {
        self.listener.listen(backlog)
            .map(TcpAcceptor::from_raw).map_err(FromError::from_error)
    }
}

pub struct TcpAcceptor {
    acceptor: socket::TcpAcceptor
}

impl TcpAcceptor {
    pub fn from_raw(acceptor: socket::TcpAcceptor) -> TcpAcceptor {
        TcpAcceptor { acceptor: acceptor }
    }
}

impl IoAcceptor for TcpAcceptor {
    type Stream = TcpStream;

    fn accept(self) -> AioEventStream<TcpStream> {
        let (producer, consumer) = EventStream::pair();

        Registration::with_opts(
            self.acceptor,
            Some(producer),
            move |io, producer, _| {
                debug!("Accepting a connection from {:?}", io);
                match AioError::from_nonblock(io.accept()) {
                    Ok(sock) => {
                        debug!("Connected to socket {:?}", sock);
                        producer.take().unwrap().send(TcpStream::from_raw(sock));
                        true
                    },
                    Err(err) => {
                        debug!("Error connecting to socket {:?}", err);
                        producer.take().unwrap().fail(err);
                        false
                    }
                }
            },
            |_, _| panic!(),
            Interest::readable(),
            PollOpt::edge()
        // FIXME: unwrap
        ).register().unwrap();

        consumer
    }
}

pub struct TcpStream {
    stream: Io<socket::TcpSocket>
}

impl TcpStream {
    pub fn from_raw(socket: socket::TcpSocket) -> TcpStream {
        TcpStream { stream: Io::new(socket) }
    }
}

impl IoReadStream for TcpStream {
    type PipeResult = ();

    fn pipe<W>(self, write: W) -> AioFuture<()>
    where W: IoWriteStream {
        self.stream.pipe(write)
    }
}

impl IoWriteStream for TcpStream {
    type Writer = <Io<socket::TcpSocket> as IoWriteStream>::Writer;

    fn on_write<W>(self, listener: W)
    where W: FnMut(&mut <TcpStream as IoWriteStream>::Writer) -> bool + 'static {
        self.stream.on_write(listener)
    }
}

