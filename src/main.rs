use std::{io, os::unix::io::AsRawFd};

fn main() {
    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    const BATCH_SIZE: usize = 1000;

    let payload = format!("{green}{}{reset}", "frog\n".repeat(BATCH_SIZE));
    let bytes = payload.into_bytes();
    let len = bytes.len();
    let fd = io::stdout().as_raw_fd();

    println!("Launching UWUPK...");

    loop {
        unsafe {
            libc::write(fd, bytes.as_ptr().cast::<libc::c_void>(), len);
        }
    }
}
