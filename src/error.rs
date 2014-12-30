use std::io::IoError;
use std::error::{FromError, Error};
use std::fmt::{mod, Show};

pub type AioResult<T> = Result<T, AioError>;

#[deriving(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AioError {
    pub desc: &'static str,
    pub kind: Kind
}

impl AioError {
    pub fn new(desc: &'static str, kind: Kind) -> AioError {
        AioError {
            desc: desc,
            kind: kind
        }
    }

    pub fn from_kind(kind: Kind) -> AioError {
        AioError {
            kind: kind,
            desc: match kind {
                Kind::Eof => "end of file",
                Kind::AddressInUse => "address in use",
                Kind::PermissionDenied => "permission denied",
                Kind::ConnectionFailed => "connection failed",
                Kind::ConnectionClosed => "connection closed",
                Kind::ConnectionRefused => "connection refused",
                Kind::ConnectionReset => "connection reset",
                Kind::ConnectionAborted => "connection aborted",
                Kind::NotConnected => "not connected",
                Kind::BrokenPipe => "broken pipe",
                Kind::PathAlreadyExists => "path already exists",
                Kind::PathDoesntExist => "path doesn't exist",
                Kind::MismatchedFileType => "mismatched file type for operation",
                Kind::TemporaryFailure => "temporary failure (resource unavailable)",
                Kind::IoUnavailable => "io unvailable on this thread",
                Kind::InvalidInput => "invalid input for this operation",
                Kind::Other => "unknown I/O error"
            }
        }
    }

    pub fn from_errno(errno: uint) -> AioError {
        FromError::from_error(IoError::from_errno(errno, false))
    }

    pub fn latest() -> AioError {
        use std::os;

        AioError::from_errno(os::errno())
    }
}

impl Show for AioError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.kind, self.desc)
    }
}

impl Error for AioError {
    fn description(&self) -> &str { self.desc }
    fn detail(&self) -> Option<String> { None }
}

impl FromError<IoError> for AioError {
    fn from_error(io: IoError) -> AioError {
        use std::io::IoErrorKind as StdKind;

        AioError {
            desc: io.desc,
            kind: match io.kind {
                StdKind::OtherIoError
                    | StdKind::ShortWrite(..)
                    | StdKind::TimedOut
                    | StdKind::NoProgress => Kind::Other,
                StdKind::EndOfFile => Kind::Eof,
                StdKind::FileNotFound
                    | StdKind::PathDoesntExist => Kind::PathDoesntExist,
                StdKind::PermissionDenied => Kind::PermissionDenied,
                StdKind::ConnectionFailed => Kind::ConnectionFailed,
                StdKind::Closed => Kind::ConnectionClosed,
                StdKind::ConnectionRefused => Kind::ConnectionRefused,
                StdKind::ConnectionReset => Kind::ConnectionReset,
                StdKind::ConnectionAborted => Kind::ConnectionAborted,
                StdKind::NotConnected => Kind::NotConnected,
                StdKind::BrokenPipe => Kind::BrokenPipe,
                StdKind::PathAlreadyExists => Kind::PathAlreadyExists,
                StdKind::MismatchedFileTypeForOperation => Kind::MismatchedFileType,
                StdKind::ResourceUnavailable => Kind::TemporaryFailure,
                StdKind::IoUnavailable => Kind::IoUnavailable,
                StdKind::InvalidInput => Kind::InvalidInput,
            }
        }
    }
}

impl FromError<AioError> for Box<Error> {
    fn from_error(aio: AioError) -> Box<Error> { box aio }
}

#[deriving(Copy, Clone, Show, PartialEq, Eq, Hash)]
pub enum Kind {
    /// End of file or socket closed
    Eof,

    /// Inet socket address or domain socket path already in use
    AddressInUse,

    /// Permissions disallowed access to this file
    PermissionDenied,

    /// Network connection failed
    ConnectionFailed,

    /// Network connection failed because the connection was closed
    ConnectionClosed,

    /// The remote server refused the connection
    ConnectionRefused,

    /// The remote server reset the connection
    ConnectionReset,

    /// The remote server terminated the connection
    ConnectionAborted,

    /// Network operation failed because it wasn't connected
    NotConnected,

    /// The operation failed because a pipe was closed
    BrokenPipe,

    /// A file already exists at this location
    PathAlreadyExists,

    /// No file was found at this location
    PathDoesntExist,

    /// This file type does not support the operation
    MismatchedFileType,

    /// The operation failed non-terminally, and retrying may succeed
    TemporaryFailure,

    /// This thread doesn't support IO
    IoUnavailable,

    /// A parameter for this operation was set incorrectly
    InvalidInput,

    /// Some other error, should be used sparingly
    Other
}

