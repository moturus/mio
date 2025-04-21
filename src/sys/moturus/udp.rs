use super::map_moturus_error;
use std::io;
use std::net::{self, SocketAddr};

pub fn bind(addr: SocketAddr) -> io::Result<net::UdpSocket> {
    let mut socket = net::UdpSocket::bind(addr)?;
    socket.set_nonblocking(true)?;

    Ok(socket)
}

pub(crate) fn only_v6(_: &net::UdpSocket) -> io::Result<bool> {
    Err(io::ErrorKind::Unsupported.into())
}
