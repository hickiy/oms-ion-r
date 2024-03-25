use wasi::wasi_unstable::{self, Fd};

fn main() {
    let fd = Fd::from_raw(wasi_unstable::STDIN_FD);
    let mut buffer = [0; 1024];
    let iov = wasi_unstable::Iovec {
        buf: buffer.as_mut_ptr(),
        len: buffer.len(),
    };
    let nread = wasi_unstable::fd_read(fd, &[iov]).unwrap();
    println!("Read {} bytes", nread);
}
