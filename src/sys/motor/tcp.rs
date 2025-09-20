use super::map_motor_error;
use std::io;
use std::net::{self, SocketAddr};

pub(crate) fn bind(addr: SocketAddr) -> io::Result<i32> {
    let fd =
        moto_rt::net::bind(moto_rt::net::PROTO_TCP, &addr.into()).map_err(map_motor_error)?;
    moto_rt::net::set_nonblocking(fd, true).map_err(map_motor_error)?;

    Ok(fd)
}

pub(crate) fn listen(listener: &net::TcpListener, max_backlog: u32) -> io::Result<()> {
    use std::os::fd::AsRawFd;

    let fd = listener.as_raw_fd();
    moto_rt::net::listen(fd, max_backlog).map_err(map_motor_error)
}

pub(crate) fn connect(addr: SocketAddr) -> io::Result<i32> {
    moto_rt::net::tcp_connect(&addr.into(), std::time::Duration::MAX, true).map_err(map_motor_error)
}

pub(crate) fn accept(listener: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
    use std::os::fd::AsRawFd;
    use std::os::fd::FromRawFd;

    let fd = listener.as_raw_fd();
    moto_rt::net::accept(fd)
        .map(|(fd, addr)| (unsafe { net::TcpStream::from_raw_fd(fd) }, addr.into()))
        .map_err(map_motor_error)
}
