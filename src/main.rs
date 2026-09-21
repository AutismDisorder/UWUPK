use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let mut data = if args.is_empty() {
        b"y\n".to_vec()
    } else {
        args.iter()
            .flat_map(|a| a.as_bytes())
            .copied()
            .chain(std::iter::once(b'\n'))
            .collect()
    };

    let (fd, len, payload_start) = {
        let fd = io::stdout().as_raw_fd();

        #[cfg(target_os = "linux")]
        {
            let ps = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
            let pad = (ps - data.as_ptr() as usize % ps) % ps;
            if pad > 0 {
                data.splice(..0, std::iter::repeat(0).take(pad));
            }
            let payload_start = (ps - data.as_ptr() as usize % ps) % ps;
            let len = data.len() - payload_start;
            (fd, len, payload_start)
        }

        #[cfg(not(target_os = "linux"))]
        {
            (fd, data.len(), 0)
        }
    };

    loop {
        #[cfg(target_os = "linux")]
        unsafe {
            let iov = libc::iovec {
                iov_base: data.as_ptr().add(payload_start) as _,
                iov_len: len,
            };
            if libc::syscall(libc::SYS_vmsplice, fd, &iov, 1, libc::SPLICE_F_GIFT) < 0 {
                libc::write(fd, data.as_ptr().add(payload_start) as _, len);
            }
        }
        #[cfg(not(target_os = "linux"))]
        unsafe {
            libc::write(fd, data.as_ptr(), data.len());
        }
    }
}
