use std::env;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let len = args.iter().map(|a| a.as_bytes().len()).sum::<usize>() + 1;
    let mut data = Vec::with_capacity(len);
    for arg in args {
        data.extend_from_slice(arg.as_bytes());
    }
    data.push(b'\n');

    let fd = io::stdout().as_raw_fd();
    let iov = libc::iovec {
        iov_base: data.as_ptr() as _,
        iov_len: data.len(),
    };

    #[cfg(target_os = "linux")]
    if unsafe { libc::syscall(libc::SYS_vmsplice, fd, &iov, 1, libc::SPLICE_F_GIFT) >= 0 } {
        loop {
            unsafe {
                libc::syscall(libc::SYS_vmsplice, fd, &iov, 1, libc::SPLICE_F_GIFT);
            }
        }
    }

    loop {
        unsafe {
            libc::write(fd, data.as_ptr() as _, data.len());
        }
    }
}
