# Lesson 23: Project: Concurrent URL Health Checker

*Part 4 · Projects* · Video 23 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Check hundreds of URLs at once with Tokio and reqwest, with a concurrency limit, timeouts and a summary report.

## Run it

Put one URL per line in a file (see `urls.txt`) and run `cargo run -- urls.txt`. Try `-c 20` for more concurrency and `--timeout-ms 5000`. `testserver.py` is the small local test server used in the video: `python3 testserver.py`, then `cargo run -- many.txt -c 1` and `-c 20` to compare speeds.

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `src/main.rs` (final): `cargo run` | Here's the whole program. The Args struct, with clap, takes the file, a concurrency limit and a timeout. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.
