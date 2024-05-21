
GH repo to blogpost https://quickwit.io/blog/performance-investigation


## Prequisite

* Rust https://www.rust-lang.org/tools/install
* https://github.com/bazhenov/cargo-export


## Run

```
cargo export target/bench -- bench

# Run SLOW
target/bench/bench_riddle

# Run FAST
FAST=TRUE target/bench/bench_riddle

# Set The number of bytes allocated before the bench
FAST=TRUE NUM_BYTES=4000000 target/bench/bench_riddle

# Trace System Calls
FAST=TRUE strace -e trace=mmap,munmap,mremap,brk target/bench/bench_riddle

# Set glibc parameters
ALLOC_MMAP_THRESHOLD_=4000000 MALLOC_TRIM_THRESHOLD_=-1 target/bench/bench_riddle

```
