# Lesson 4: Ownership

*Part 1 · Beginner* · Video 4 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

The idea that makes Rust different: memory safety without a garbage collector.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `mistakes/step01.rs` (does not compile, on purpose) | Let's see a move. We create a String called s1. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | If you really want two independent copies, call clone. |
| 3 | `mistakes/step03.rs` (does not compile, on purpose) | Passing a value to a function works the same way as assignment. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Let's actually watch values being dropped. |
| 5 | `src/main.rs` (final): `cargo run` | One way to keep using a value is to give ownership back. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Predict, then check: which lines fail? `let v = vec![1, 2]; let w = v; println!("{:?}", v);`
- Make the `print_name` example compile in two different ways
- Add a third `Noisy` in a nested block: predict the drop order first
- Explain in one sentence why `i32` copies but `String` moves
