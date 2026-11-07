# Lesson 12: Lifetimes

*Part 2 · Intermediate* · Video 12 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

How Rust proves every reference is valid, and when you need to help it.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `mistakes/step01.rs` (does not compile, on purpose) | First, the problem lifetimes solve. R is declared in the outer scope. |
| 2 | `mistakes/step02.rs` (does not compile, on purpose) | Now a function. Longest takes two string slices, and returns whichever is longer. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | We add a lifetime parameter, written tick a, in angle brackets, and use it on both inputs and the output. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Structs that hold references need lifetimes too. |
| 5 | `src/main.rs` (final): `cargo run` | Two last points. The special lifetime tick static means the data lives for the entire program. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `fn first_or<'a>(v: &'a [String], fallback: &'a str) -> &'a str`
- Build `struct Tokenizer<'a> { input: &'a str }` with `fn words(&self) -> Vec<&'a str>`
- Uncomment the `outside` line and read the error carefully
- Rewrite `Excerpt` to own its data, and compare
