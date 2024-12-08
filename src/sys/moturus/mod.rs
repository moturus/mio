macro_rules! os_required {
    () => {
        panic!("mio must be compiled with `os-poll` to run.")
    };
}

mod selector;
pub(crate) use self::selector::{event, Event, Events, Selector};

mod waker;
pub(crate) use waker::Waker;

cfg_net! {
    pub(crate) mod tcp;
    pub(crate) mod udp;
}

cfg_io_source! {
    use std::io;
    use std::os::fd::RawFd;
    use crate::{Registry, Token, Interest};

    pub(crate) struct IoSourceState;

    impl IoSourceState {
        pub fn new() -> IoSourceState {
            IoSourceState
        }

        pub fn do_io<T, F, R>(&self, f: F, io: &T) -> io::Result<R>
        where
            F: FnOnce(&T) -> io::Result<R>,
        {
            // We don't hold state, so we can just call the function and
            // return.
            f(io)
        }
    }

    impl IoSourceState {
        pub fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interest: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            registry.selector().register(token, interest, fd)
        }

        pub fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interest: Interest,
            fd: RawFd,
        ) -> io::Result<()> {
            registry.selector().reregister(token, interest, fd)
        }

        pub fn deregister(&mut self, registry: &Registry, fd: RawFd) -> io::Result<()> {
            registry.selector().deregister(fd)
        }
    }
}

fn map_moturus_error(err: moto_rt::ErrorCode) -> std::io::Error {
    use moto_rt::error::*;

    use std::io::ErrorKind;

    let kind: ErrorKind = match err {
        E_ALREADY_IN_USE => ErrorKind::AlreadyExists,
        E_NOT_FOUND => ErrorKind::NotFound,
        E_TIMED_OUT => ErrorKind::TimedOut,
        E_NOT_IMPLEMENTED => ErrorKind::Unsupported,
        E_UNEXPECTED_EOF => ErrorKind::UnexpectedEof,
        E_INVALID_ARGUMENT => ErrorKind::InvalidData,
        E_NOT_READY => ErrorKind::WouldBlock,
        _ => ErrorKind::Other,
    };

    std::io::Error::from(kind)
}
