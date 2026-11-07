# Lesson 6: Structs and Methods

*Part 1 · Beginner* · Video 6 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Group related data into your own types, and give them behaviour with impl blocks.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | A struct has a name, and named fields, each with a type. |
| 2 | `mistakes/step02.rs` (does not compile, on purpose) | Printing the fields one by one is tedious. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | The allow dead code attribute just silences a warning, because this example only reads the fields through Debug. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | There are two other shapes of struct. A tuple struct has fields with no names, accessed by position, like dot zero. |
| 5 | `examples/step05.rs`: `cargo run --example step05` | Now methods. We put them in an impl block for the type. |
| 6 | `src/main.rs` (final): `cargo run` | The first parameter decides how a method treats the struct, using exactly the rules from the last two lessons. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Build `struct BankAccount { owner, balance }` with `deposit`, `withdraw` and `balance`
- Make `withdraw` refuse to go below zero and return a `bool`
- Add `fn new(owner: &str) -> Self`
- Add a `Circle` struct with an `area` method using `std::f64::consts::PI`
