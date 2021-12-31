#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::AssetState;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub(crate) use wasm::AssetState;

use crate::ctx;
use alloc::{string::String, vec::Vec};

/// Requests a read of an asset. This produces an AssetRead event with the result of the read once
/// it has completed.
///
/// ## Platform-specific
///
/// - **Non-web:** The path is relative to the current working directory.
/// - **Web:** The path is relative to the current url's root.
pub fn request_read(relative_path: &str) {
    ctx().assets().push_read(relative_path);
}

pub(crate) trait AssetStateContract {
    /// Creates a new asset state.
    fn init() -> Self;

    /// Pushes a read request to the queue. Relative to the current working directory.
    fn push_read(&mut self, relative_path: &str);

    /// Pops the next available read off the queue, returning None if there are no finished reads
    /// available.
    fn try_pop_read(&mut self) -> Option<Result<Asset, LoaderError>>;
}

/// Represents a binary blob loaded from an external source.
#[derive(Clone, Debug)]
pub struct Asset {
    /// The path used to query for this asset.
    pub relative_path: String,
    /// The contents of the asset as bytes.
    pub contents: Vec<u8>,
}

impl Asset {
    /// Creates a new asset.
    pub fn new(relative_path: String, contents: Vec<u8>) -> Asset {
        Asset {
            relative_path,
            contents,
        }
    }
}

/// A list specifying general categories of I/O error.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LoaderError {
    /// An entity was not found, often a file.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// The connection was refused by the remote server.
    ConnectionRefused,
    /// The connection was reset by the remote server.
    ConnectionReset,
    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,
    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,
    /// The operation failed because a pipe was closed.
    BrokenPipe,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// A parameter was incorrect.
    InvalidInput,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: LoaderError::InvalidInput
    InvalidData,
    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,
    /// An error returned when an operation could not be completed because a
    /// call to `write` returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    WriteZero,
    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,
    /// This operation is unsupported on this platform.
    ///
    /// This means that the operation can never succeed.
    Unsupported,
    // ErrorKinds which are primarily categorisations for OS error
    // codes should be added above.
    //
    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,
    /// An operation could not be completed, because it failed
    /// to allocate enough memory.
    OutOfMemory,
    // "Unusual" error kinds which do not correspond simply to (sets
    // of) OS error codes, should be added just above this comment.
    // `Other` and `Uncategorised` should remain at the end:
    //
    /// A custom error that does not fall under any other I/O error kind.
    ///
    /// This can be used to construct your own `Error`s that do not match any
    /// `ErrorKind`.
    ///
    /// This `ErrorKind` is not used by the standard library.
    ///
    /// Errors from the standard library that do not fall under any of the I/O
    /// error kinds cannot be `match`ed on, and will only match a wildcard (`_`) pattern.
    /// New `ErrorKind`s might be added in the future for some of those.
    Other,
}
