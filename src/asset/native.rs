use crate::asset::{Asset, LoaderError};
use crate::sync::{make as spsc_make, Consumer, Producer};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use std::fs::File;
use std::{io, io::Read};
use std::{thread, thread::JoinHandle};

pub struct AssetState {
    handle: JoinHandle<()>,
    read_request_sender: Producer<String>,
    read_result_receiver: Consumer<Result<Asset, LoaderError>>,
}

impl AssetState {
    pub fn init() -> AssetState {
        let (read_request_sender, read_request_receiver) = spsc_make(256);
        let (read_result_sender, read_result_receiver) = spsc_make(256);

        let handle = thread::spawn(move || {
            while let Some(relative_path) = read_request_receiver.try_pop() {
                read_result_sender.push(read(relative_path));
            }
        });

        AssetState {
            handle,
            read_request_sender,
            read_result_receiver,
        }
    }

    /// Pushes a read request to the queue. Relative to the current working directory.
    pub fn push_read(&mut self, relative_path: &str) {
        self.read_request_sender.push(relative_path.to_string())
    }

    /// Pops the next available read off the queue, returning None if there are no finished reads
    /// available.
    pub fn try_pop_read(&mut self) -> Option<Result<Asset, LoaderError>> {
        self.read_result_receiver.try_pop()
    }
}

/// Relative to the current working directory.
fn read(relative_path: String) -> Result<Asset, LoaderError> {
    let mut file = match File::open(&relative_path) {
        Ok(file) => file,
        Err(error) => return Err(error.kind().into()),
    };
    let mut contents = Vec::new();
    if let Err(error) = file.read_to_end(&mut contents) {
        return Err(error.kind().into());
    }
    Ok(Asset::new(relative_path, contents))
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
