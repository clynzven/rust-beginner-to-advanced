# Lesson 9: Error Handling

*Part 2 · Intermediate* · Video 9 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Result, the question mark operator, custom error types, thiserror and anyhow.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Parse returns a Result: Ok with the number, or Err with a ParseIntError. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Now the question mark operator. Read config returns a Result. |
| 3 | `mistakes/step03.rs` (does not compile, on purpose) | Main can return a Result too. Box dyn Error means any kind of error, which is convenient at the top level of a program. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | For your own code, you'll want your own error type, and an enum is perfect: one variant per thing that can go wrong, with data attached. |
| 5 | `examples/step05.rs`: `cargo run --example step05` | Writing Display by hand gets repetitive, so libraries use the thiserror crate. |
| 6 | `src/main.rs` (final): `cargo run` | For applications, as opposed to libraries, the anyhow crate is the usual choice. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `fn average(path: &str) -> anyhow::Result<f64>` for a file of numbers, one per line
- Report the line number of a bad value using `.with_context`
- Create a `thiserror` enum for a login: `UnknownUser`, `WrongPassword`, `Locked { minutes }`
- Find every `unwrap()` in your earlier exercises and replace it with `?`
