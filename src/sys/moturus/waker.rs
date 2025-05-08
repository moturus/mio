use super::map_moturus_error;
use crate::sys::Selector;
use crate::Token;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

#[derive(Debug)]
pub struct Waker {
    fd: OwnedFd,
}

impl Waker {
    pub fn new(selector: &Selector, token: Token) -> io::Result<Waker> {
        // SAFETY: `moto_rt::poll::new()` ensures the fd is valid.
        let fd = unsafe { OwnedFd::from_raw_fd(moto_rt::poll::new().map_err(map_moturus_error)?) };
        selector.register(token, crate::Interest::READABLE, fd.as_raw_fd())?;

        Ok(Waker { fd })
    }

    pub fn wake(&self) -> io::Result<()> {
        moto_rt::poll::wake(self.fd.as_raw_fd()).map_err(map_moturus_error)
    }
}
