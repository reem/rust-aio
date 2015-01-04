//! The AioError type, AioResult alias, and associated utitilies.

use std::io::IoError;
use std::error::{FromError, Error};
use std::fmt::{self, Show};

use mio::{MioError, MioErrorKind};
use nix::{SysError};

/// The Result alias used throughout `aio`.
pub type AioResult<T> = Result<T, AioError>;

/// The type used to indicate errors in IO operations.
///
/// It contains a static string description of the error, as well
/// as a tag indicating the kind of the error. Mostly, `desc` should
/// be used only for logging and you should always match on `kind`.
// FIXME: This error type is 3 words. Benchmark and see if an internal box is faster.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct AioError {
    /// A string description of the error meant for logging.
    pub desc: &'static str,

    /// The type of this error, meant for disambiguating the error.
    pub kind: Kind
}

impl AioError {
    /// Create a new AioError from the constituent parts.
    pub fn new(desc: &'static str, kind: Kind) -> AioError {
        AioError {
            desc: desc,
            kind: kind
        }
    }

    /// Create a new AioError with the canonical error description.
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
                Kind::WouldBlock => "operation would block",
                Kind::Other => "unknown I/O error"
            }
        }
    }

    /// Create an AioError from a system errno.
    ///
    /// This uses the same decoding as `std::io::IoError` to
    /// figure out the error kind.
    pub fn from_errno(errno: uint) -> AioError {
        FromError::from_error(IoError::from_errno(errno, false))
    }

    /// Get the AioError best representing the latest errno.
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

impl FromError<MioError> for AioError {
    fn from_error(mio: MioError) -> AioError {
        match mio.kind {
            MioErrorKind::Eof => AioError::from_kind(Kind::Eof),
            MioErrorKind::WouldBlock => AioError::from_kind(Kind::WouldBlock),
            MioErrorKind::AddrInUse => AioError::from_kind(Kind::AddressInUse),
            MioErrorKind::BufUnderflow => AioError::new("wrote or read too little", Kind::Other),
            MioErrorKind::BufOverflow => AioError::new("wrote or read too much", Kind::Other),
            MioErrorKind::EventLoopTerminated => AioError::new("event loop terminated", Kind::Other),
            MioErrorKind::OtherError => AioError::from_kind(Kind::Other),
        }
    }
}

impl FromError<SysError> for AioError {
    fn from_error(sys: SysError) -> AioError {
        FromError::from_error(MioError::from_sys_error(sys))
    }
}

impl FromError<AioError> for Box<Error> {
    fn from_error(aio: AioError) -> Box<Error> { box aio }
}

/// The kind of an AioError
#[derive(Copy, Clone, Show, PartialEq, Eq, Hash)]
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

    /// This operation was requested to be non-blocking but would block
    WouldBlock,

    /// Some other error, should be used sparingly
    Other
}

