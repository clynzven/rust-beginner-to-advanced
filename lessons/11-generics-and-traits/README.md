# Lesson 11: Generics and Traits

*Part 2 · Intermediate* · Video 11 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Write code once for many types, and describe shared behaviour with traits.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Here's a function that finds the largest i32 in a slice. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Structs can be generic too. Point of T has an x and a y of the same type T. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Now let's define our own trait. A trait lists method signatures that a type must provide. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Traits really shine as bounds on generic functions. |
| 5 | `src/main.rs` (final): `cargo run` | In practice, you'll mostly implement standard library traits, so your types work like built-in ones. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write a `trait Shape { fn area(&self) -> f64; fn name(&self) -> String { ... } }`
- Implement it for `Circle` and `Square`, then write `fn print_area(s: &impl Shape)`
- Make a generic `struct Pair<T>` with a `fn larger(&self) -> &T` where `T: PartialOrd`
- Implement `Display` for `Point<T>` where `T: Display`
