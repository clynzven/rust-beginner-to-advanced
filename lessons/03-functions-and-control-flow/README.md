# Lesson 3: Functions and Control Flow

*Part 1 · Beginner* · Video 3 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Functions, expressions vs statements, if, loop, while and for.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Functions are declared with fn, then a name in snake case, then the parameters. |
| 2 | `mistakes/step02.rs` (does not compile, on purpose) | Here's a block used as an expression: inside the braces we make x, and the last line, x times x plus one, becomes the value of y. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | I've removed the semicolon, and simplified main. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Rust has three loops. Loop repeats forever until you break. |
| 5 | `src/main.rs` (final): `cargo run` | Let's put it together with FizzBuzz. For numbers one to fifteen: multiples of fifteen print FizzBuzz, multiples of three print Fizz, multiples of five print ... |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `fn celsius_to_fahrenheit(c: f64) -> f64`
- Write `fn fib(n: u32) -> u64` using a loop (not recursion)
- Print a multiplication table for 1 to 9 with nested `for` loops
- Change FizzBuzz so the limit comes from a variable
