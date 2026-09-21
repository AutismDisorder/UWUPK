use std::env;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;

fn main() {
    let args = env::args_os().skip(1).collect::<Vec<_>>();
    let out = if args.is_empty() {
        b"y\n".to_vec()
    } else {
        let mut buf =
            Vec::with_capacity(args.iter().map(|a| a.as_bytes().len()).sum::<usize>() + 1);
        for arg in &args {
            buf.extend_from_slice(arg.as_bytes());
        }
        buf.push(b'\n');
        buf
    };

    const TARGET: usize = 256 * 1024;
    let mut amp = Vec::with_capacity(TARGET);

    unsafe {
        let olen = out.len();
        std::ptr::copy_nonoverlapping(out.as_ptr(), amp.as_mut_ptr(), olen);
        amp.set_len(olen);

        while amp.len() < TARGET {
            let llen = amp.len();
            let copy_len = std::cmp::min(llen, TARGET - llen);
            std::ptr::copy_nonoverlapping(amp.as_ptr(), amp.as_mut_ptr().add(llen), copy_len);
            amp.set_len(llen + copy_len);
        }

        let fd = std::io::stdout().as_raw_fd();
        let iov = libc::iovec {
            iov_base: amp.as_ptr() as _,
            iov_len: amp.len(),
        };

        if libc::syscall(libc::SYS_vmsplice, fd, &iov, 1, libc::SPLICE_F_GIFT) >= 0 {
            loop {
                libc::syscall(libc::SYS_vmsplice, fd, &iov, 1, libc::SPLICE_F_GIFT);
            }
        }

        loop {
            libc::write(fd, amp.as_ptr() as _, amp.len());
        }
    }
}
