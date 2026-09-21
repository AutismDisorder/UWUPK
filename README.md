# Fastest "frog" printer

**UWUPK** has one mission, printing *frog* faster than any other *frog* printer there is in existence. It prints *frog*-s' so fast that benchmarks would cause **immeasurable** degradation (alas the avoidance).

---

## Running UWUPK

*compile it:*
```bash
git clone https://github.com/AutismDisorder/UWUPK && cd UWUPK
cargo run --release
```

--- 

## How is it faster than a println loop?

**Boring and fast explanation**

*UWUPK* is a multi-threaded engine that utilizes Double Buffering and references the same single piece of memory across all threads, avoiding missing caches and keeps CPU's L1/L2 caches focused on the data. It reuses the same memory allocation forever to ensure Rust's Drop checker never does any work.

**Not-so-boring and longer explanation. (Keep any criticism)**

I'll feel free to not pack something fancy to your expectation:

println! is a complex engine that:
```
- parses format strings
- validates Unicode at runtime.
- protects it's call by a global mutex.
- introduces unnecessary overhead printing frog-s'
```
*UWUPK* uses libc::write, a direct system call that bypasses userspace locks entirely, allowing all threads to hammer the kernel's write queue simultaneously. And it also uses a pre-computed byte array. By converting the *"frog"* string and ANSI color codes into a Vec<u8> at startup, the CPU avoids all formatting logic and simply moves contiguous blocks of bytes from memory to the kernel.

Every system call triggers a "Context Switch" from User Mode to Kernel Mode, which is computationally expensive. UWUPK minimizes this by:

    Batching: Writing 1,000 frog-s' per call instead of one, reducing syscall frequency by 99.9%.
    Raw Pointer Access: Using *const u8 to bypass the bounds-checking overhead typically found in Rust slices.

*UWWPK* further improves performance by using an Arc-wrapped SendPtr that ensures that all threads reference the same physical memory address. This maximizes the L1/L2 cache hit rate, as the "frog" payload remains resident in the fastest CPU cache rather than being fetched from slower RAM. Additionally, the hot loop is entirely allocation-free, meaning the heap allocator and Drop checker never trigger.

Via rayon, *UWUPK* employs a work-stealing scheduler that saturates every logical core on the system. By mapping exactly one worker to each core, we ensure 100% CPU utilization without the overhead of excessive OS thread context switching.

### For colors:
Instead of sending separate commands for colors, *UWUPK* bakes the ANSI escape codes (\x1b[32m) directly into the binary stream. This reduces the number of control instructions the terminal emulator has to parse, shifting the rendering load to the GPU.


# Release me T_T
This is Extreme Over-engineering of the Hyper-Frog Rendering Framework.
I need to find the keys out of my basement, I need to go touch grass! 

**Warning**
This may *"melt"* your terminal. You've been warned.
I also warn you, my AGPL license is ragebaitingly *serious*. I assume that it's working.
