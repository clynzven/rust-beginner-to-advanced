# Lesson 17: Async Rust with Tokio

*Part 3 · Advanced* · Video 17 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Futures, async and await, concurrent tasks, timeouts and select.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Here's a fake fetch function that waits for some milliseconds, to simulate a network request. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Now with tokio join. It takes several futures and runs them concurrently, waiting for all of them. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Join is for a fixed number of futures. For a dynamic number, spawn tasks. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Two powerful tools. Timeout wraps any future, and gives up if it takes too long. |
| 5 | `src/main.rs` (final): `cargo run` | One trap to avoid. Inside async code, never call something that blocks the thread, like standard thread sleep, heavy computation, or a blocking file read. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `async fn retry<F>(attempts: u32, f: F)` that retries a failing future with a delay
- Spawn 1,000 tasks that each sleep 100 ms: how long does it take in total?
- Use `select!` to stop a worker when Ctrl+C is pressed (`tokio::signal::ctrl_c`)
- Rewrite the channel example to use `tokio::sync::broadcast` with two receivers
