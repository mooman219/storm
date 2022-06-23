use crate::asset::{Asset, AssetRequest, AssetStateContract, LoaderError};
use crate::sync::{make as spsc_make, Consumer, Producer, Signal};
use crate::App;
use alloc::{sync::Arc, vec::Vec};
use std::fs::File;
use std::{io, io::Read};
use std::{thread, thread::JoinHandle};

pub(crate) struct AssetState<A: App> {
    handle: JoinHandle<()>,
    signal: Arc<Signal>,
    pending: Vec<AssetRequest<A>>,
    read_request_sender: Producer<AssetRequest<A>>,
    read_result_receiver: Consumer<AssetRequest<A>>,
}

impl<A: App> AssetStateContract<A> for AssetState<A> {
    fn init() -> Self {
        let (read_request_sender, read_request_receiver): (
            Producer<AssetRequest<A>>,
            Consumer<AssetRequest<A>>,
        ) = spsc_make(128);
        let (read_result_sender, read_result_receiver): (
            Producer<AssetRequest<A>>,
            Consumer<AssetRequest<A>>,
        ) = spsc_make(128);

        let signal = Arc::new(Signal::new());
        let handle_signal = signal.clone();

        let handle = thread::spawn(move || loop {
            while let Some(mut request) = read_request_receiver.try_pop() {
                for asset in &mut request.assets {
                    read(asset);
                }
                read_result_sender.push(request);
            }
            handle_signal.wait();
        });

        AssetState {
            handle,
            signal,
            pending: Vec::new(),
            read_request_sender,
            read_result_receiver,
        }
    }

    fn read(&mut self, request: AssetRequest<A>) {
        if let Some(path) = self.read_request_sender.try_push(request) {
            self.pending.push(path);
        } else {
            self.signal.notify();
        }
    }

    fn next(&mut self) -> Option<AssetRequest<A>> {
        // Send the backlog to be read.
        while let Some(path) = self.pending.pop() {
            if let Some(path) = self.read_request_sender.try_push(path) {
                self.pending.push(path);
                break;
            } else {
                self.signal.notify();
            }
        }
        // Process the finished requests.
        self.read_result_receiver.try_pop()
    }
}

/// Relative to the current working directory.
fn read(asset: &mut Asset) {
    let mut file = match File::open(&asset.relative_path) {
        Ok(file) => file,
        Err(error) => {
            asset.result = Err(error.kind().into());
            return;
        }
    };
    let mut contents = Vec::new();
    if let Err(error) = file.read_to_end(&mut contents) {
        asset.result = Err(error.kind().into());
        return;
    }
    asset.result = Ok(contents);
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
