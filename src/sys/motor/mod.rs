pub use std::os::motor::map_motor_error;

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

