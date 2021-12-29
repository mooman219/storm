use crate::loader::LoaderError;
use alloc::vec::Vec;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct LoaderState {}

impl LoaderState {
    pub fn init() -> LoaderState {
        LoaderState {}
    }
}

/// Relative to the current working directory.
fn read(relative_path: &str) -> Result<Vec<u8>, LoaderError> {
    let mut file = match File::open(relative_path) {
        Ok(file) => file,
        Err(error) => return Err(error.kind().into()),
    };
    let mut buffer = Vec::new();
    if let Err(error) = file.read_to_end(&mut buffer) {
        return Err(error.kind().into());
    }
    Ok(buffer)
}

impl From<io::ErrorKind> for LoaderError {
    fn from(error: io::ErrorKind) -> Self {
        match error {
            io::ErrorKind::NotFound => LoaderError::NotFound,
            io::ErrorKind::PermissionDenied => LoaderError::PermissionDenied,
            io::ErrorKind::ConnectionRefused => LoaderError::ConnectionRefused,
            io::ErrorKind::ConnectionReset => LoaderError::ConnectionReset,
            io::ErrorKind::ConnectionAborted => LoaderError::ConnectionAborted,
            io::ErrorKind::NotConnected => LoaderError::NotConnected,
            io::ErrorKind::AddrInUse => LoaderError::AddrInUse,
            io::ErrorKind::AddrNotAvailable => LoaderError::AddrNotAvailable,
            io::ErrorKind::BrokenPipe => LoaderError::BrokenPipe,
            io::ErrorKind::AlreadyExists => LoaderError::AlreadyExists,
            io::ErrorKind::WouldBlock => LoaderError::WouldBlock,
            io::ErrorKind::InvalidInput => LoaderError::InvalidInput,
            io::ErrorKind::InvalidData => LoaderError::InvalidData,
            io::ErrorKind::TimedOut => LoaderError::TimedOut,
            io::ErrorKind::WriteZero => LoaderError::WriteZero,
            io::ErrorKind::Interrupted => LoaderError::Interrupted,
            io::ErrorKind::Unsupported => LoaderError::Unsupported,
            io::ErrorKind::UnexpectedEof => LoaderError::UnexpectedEof,
            io::ErrorKind::OutOfMemory => LoaderError::OutOfMemory,
            _ => LoaderError::Other,
        }
    }
}
