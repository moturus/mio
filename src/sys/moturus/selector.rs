use super::map_moturus_error;
use crate::{Interest, Token};
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::time::Duration;

pub type Event = moto_rt::poll::Event;
pub type Events = Vec<Event>;

/// Unique id for use as `SelectorId`.
#[cfg(debug_assertions)]
static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

#[derive(Debug)]
pub struct Selector {
    fd: moto_rt::RtFd,
    #[cfg(debug_assertions)]
    id: usize,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let fd = moto_rt::poll::new().map_err(map_moturus_error)?;
        Ok(Self {
            fd,
            #[cfg(debug_assertions)]
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        })
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        todo!()
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        let abs_timeout = timeout.map(|d| moto_rt::time::Instant::now() + d);
        events.clear();
        moto_rt::poll::wait(self.fd, events.as_mut_ptr(), events.capacity(), abs_timeout)
            .map(|n_events| unsafe { events.set_len(n_events as usize) })
            .map_err(map_moturus_error)
    }

    pub fn register(&self, token: Token, interest: Interest, fd: RawFd) -> io::Result<()> {
        let mut rt_interest = 0;
        if interest.is_readable() {
            rt_interest |= moto_rt::poll::POLL_READABLE;
        }
        if interest.is_writable() {
            rt_interest |= moto_rt::poll::POLL_WRITABLE;
        }
        moto_rt::poll::add(self.fd, fd, token.0 as u64, rt_interest).map_err(map_moturus_error)
    }

    pub fn reregister(&self, token: Token, interest: Interest, fd: RawFd) -> io::Result<()> {
        let mut rt_interest = 0;
        if interest.is_readable() {
            rt_interest |= moto_rt::poll::POLL_READABLE;
        }
        if interest.is_writable() {
            rt_interest |= moto_rt::poll::POLL_WRITABLE;
        }
        moto_rt::poll::set(self.fd, fd, token.0 as u64, rt_interest).map_err(map_moturus_error)
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        moto_rt::poll::del(self.fd, fd).map_err(map_moturus_error)
    }
}

cfg_io_source! {
    #[cfg(debug_assertions)]
    impl Selector {
        pub fn id(&self) -> usize {
            self.id
        }
    }
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub mod event {
    use crate::sys::Event;
    use crate::Token;
    use std::fmt;

    pub fn token(event: &Event) -> Token {
        Token(event.token as usize)
    }

    pub fn is_readable(event: &Event) -> bool {
        event.events & moto_rt::poll::POLL_READABLE != 0
    }

    pub fn is_writable(event: &Event) -> bool {
        event.events & moto_rt::poll::POLL_WRITABLE != 0
    }

    pub fn is_error(event: &Event) -> bool {
        event.events & moto_rt::poll::POLL_ERROR != 0
    }

    pub fn is_read_closed(event: &Event) -> bool {
        event.events & moto_rt::poll::POLL_READ_CLOSED != 0
    }

    pub fn is_write_closed(event: &Event) -> bool {
        event.events & moto_rt::poll::POLL_WRITE_CLOSED != 0
    }

    pub fn is_priority(_: &Event) -> bool {
        false
    }

    pub fn is_aio(_: &Event) -> bool {
        false
    }

    pub fn is_lio(_: &Event) -> bool {
        false
    }

    pub fn debug_details(f: &mut fmt::Formatter<'_>, event: &Event) -> fmt::Result {
        use std::fmt::Debug;

        event.fmt(f)
    }
}