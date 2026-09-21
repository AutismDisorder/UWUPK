use std::io::{self};
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;

fn main() {
    const BATCH_SIZE: usize = 1000;

    let cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let green = "\x1b[32m";
    let reset = "\x1b[0m";

    let payload = format!("{green}{}{reset}", "frog\n".repeat(BATCH_SIZE));

    let bytes: Arc<Vec<u8>> = Arc::new(payload.into_bytes());
    let len = bytes.len();
    let fd = io::stdout().as_raw_fd();

    println!("Launching UWUPK on {} cores...", cores);

    rayon::scope(|s| {
        for _ in 0..cores {
            let bytes_arc = Arc::clone(&bytes);

            s.spawn(move |_| {
                loop {
                    unsafe {
                        libc::write(fd, bytes_arc.as_ptr().cast::<libc::c_void>(), len);
                    }
                }
            });
        }
    });
}
