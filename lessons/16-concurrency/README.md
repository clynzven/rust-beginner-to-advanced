# Lesson 16: Fearless Concurrency

*Part 3 · Advanced* · Video 16 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Threads, channels, Arc and Mutex, and scoped threads, with data races caught at compile time.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Thread spawn starts a new operating system thread, running the closure we give it. |
| 2 | `mistakes/step02.rs` (does not compile, on purpose) | Now, what if the thread uses a variable from main? |
| 3 | `examples/step03.rs`: `cargo run --example step03` | One safe way for threads to cooperate is message passing. |
| 4 | `mistakes/step04.rs` (does not compile, on purpose) | Now shared state. Can we share an Rc between threads, like we did between services last lesson? |
| 5 | `examples/step05.rs`: `cargo run --example step05` | The thread-safe versions are Arc, which is an atomic reference count, and Mutex, which allows only one thread at a time to access the data. |
| 6 | `src/main.rs` (final): `cargo run` | Finally, scoped threads. Thread scope guarantees that every thread spawned inside it finishes before the scope ends. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Use 4 threads and a channel to find all primes below 100,000
- Build a shared `Arc<Mutex<HashMap<String, u32>>>` word counter fed by several threads
- Rewrite the counter with `AtomicU64` and compare
- Try `rayon`: `numbers.par_iter().sum()`
