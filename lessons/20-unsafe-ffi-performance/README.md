# Lesson 20: Unsafe, FFI and Performance

*Part 3 · Advanced* · Video 20 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Raw pointers, safe abstractions over unsafe code, calling C, and measuring speed.

## Run it

Needs a C compiler (gcc, clang or MSVC), used by `build.rs`. Then `cargo run --release`.

## Exercises

- Write a safe wrapper for C's `abs` and `atoi`
- Add `double c_max(const double*, size_t)` to `stats.c` and call it from Rust
- Benchmark `String` concatenation with and without `with_capacity`
- Run Miri (`cargo +nightly miri run`) on the raw pointer example with `ptr.add(10)`
