# Lesson 2: Variables and Types

*Part 1 · Beginner* · Video 2 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Immutability by default, shadowing, constants, and Rust's scalar and compound types.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `mistakes/step01.rs` (does not compile, on purpose) | Let's see immutability in action. We set x to five and print it. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Let's follow the advice and add mut. Now we're telling the compiler, and anyone reading the code, that x is meant to change. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Now shadowing. Spaces starts as a piece of text with three spaces. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Here are the types in action. A is a signed thirty-two bit integer, minus forty-two. |
| 5 | `mistakes/step05.rs` (does not compile, on purpose) | What happens if a number overflows? A u eight can't go above two hundred and fifty-five, so let's try adding one. |
| 6 | `src/main.rs` (final): `cargo run` | When overflow is possible, Rust gives you methods that make your intent explicit. |
| 7 | `mistakes/step07.rs` (does not compile, on purpose) | Now the two compound types. A tuple groups values of different types. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Convert a temperature in Celsius to Fahrenheit using `f64`
- Make a `u8` counter and use `checked_add` in a loop of 300 steps
- Store an RGB colour in a tuple and destructure it
- Use `.get(index)` on an array instead of `[index]`, and print the result with `{:?}`
