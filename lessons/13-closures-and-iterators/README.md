# Lesson 13: Closures and Iterators

*Part 2 · Intermediate* · Video 13 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Anonymous functions that capture their environment, and lazy, chainable data pipelines.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | A closure is written with vertical bars around its parameters, followed by the body. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Apply twice takes any closure that implements Fn, taking an i32 and returning one, and calls it twice. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Now iterators. Calling iter gives an iterator of references to the elements. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Here's a more realistic pipeline over a list of sales. |
| 5 | `src/main.rs` (final): `cargo run` | You can make your own iterators by implementing the Iterator trait. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Using only iterator methods, find the sum of squares of odd numbers from 1 to 100
- Turn `"the cat sat on the mat"` into a `HashMap` of word lengths
- Write `fn compose(f, g) -> impl Fn(i32) -> i32` that returns `g(f(x))`
- Implement a `Fibonacci` iterator, and print the first 10 even Fibonacci numbers
