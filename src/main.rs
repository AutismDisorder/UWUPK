use rayon;
use std::io;
use std::os::unix::io::AsRawFd;
use std::sync::Arc;
use std::thread;

struct SendPtr(*const u8);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

impl SendPtr {
    fn get(&self) -> *const u8 {
        self.0
    }
}

fn main() {
    let cores = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let green = "\x1b[32m";
    let reset = "\x1b[0m";
    let payload = format!("{}{}{}", green, "frog\n".repeat(1000), reset);
    let bytes = payload.into_bytes();
    let len = bytes.len();

    let shared_ptr = Arc::new(SendPtr(bytes.as_ptr()));
    let fd = io::stdout().as_raw_fd();

    println!("Launching Unsafe Frog Driver on {} cores...", cores);

    rayon::scope(|s| {
        for _ in 0..cores {
            let ptr_clone = Arc::clone(&shared_ptr);

            s.spawn(move |_| {
                loop {
                    unsafe {
                        libc::write(fd, ptr_clone.get() as *const libc::c_void, len);
                    }
                }
            });
        }
    });
}
