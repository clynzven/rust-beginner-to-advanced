# Lesson 7: Enums and Pattern Matching

*Part 1 · Beginner* · Video 7 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Model choices with enums, handle every case with match, and say goodbye to null with Option.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | An enum lists its variants. Direction can be North, East, South or West, and nothing else. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Here's where Rust enums get powerful: each variant can hold different data. |
| 3 | `mistakes/step03.rs` (does not compile, on purpose) | Here's the real superpower. What if I forget to handle a variant? |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Rust has no null. Instead, a value that might be missing has the type Option, which is just an enum with two variants: Some, holding a value, and None. |
| 5 | `examples/step05.rs`: `cargo run --example step05` | Writing a full match for one interesting case gets wordy, so Rust has two shortcuts. |
| 6 | `src/main.rs` (final): `cargo run` | Patterns can do a lot more than match a single value. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Model a `Shape` enum (`Circle { r }`, `Rect { w, h }`, `Triangle { b, h }`) with an `area()` method
- Write `fn divide(a: f64, b: f64) -> Option<f64>` that returns `None` for zero
- Use `let else` to parse two numbers from a string like `"12,30"`
- Add a new `Message` variant and follow the compiler errors
