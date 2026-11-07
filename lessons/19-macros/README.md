# Lesson 19: Macros

*Part 3 · Advanced* · Video 19 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Declarative macros with macro_rules, and how derive macros like serde save you thousands of lines.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Macro rules defines a declarative macro. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | The real power is repetition. Dollar, parentheses, comma star means: zero or more expressions, separated by commas. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Macros can also generate items: structs, impl blocks, whole functions. |
| 4 | `src/main.rs` (final): `cargo run` | The other kind of macro is procedural. These are Rust functions that receive your code as tokens and output new code. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `assert_between!(x, lo, hi)` that panics with a helpful message
- Write `enum_str!` that generates an enum and a `fn as_str(&self)`
- Use `cargo expand` (install with `cargo install cargo-expand`) to see what your macros generate
- Parse a JSON array of orders, and print the total quantity per item
