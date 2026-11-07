# Lesson 1: Hello, Rust

*Part 1 · Beginner* · Video 1 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Install the toolchain, create your first project, and understand what Cargo does for you.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Here's the code Cargo generated. Fn main declares the main function, which is where every Rust program starts. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Let's make it ours. Comments start with two slashes. |
| 3 | `mistakes/step03.rs` (does not compile, on purpose) | Now let's make a mistake on purpose, because reading compiler errors is one of the most valuable skills in Rust. |
| 4 | `src/main.rs` (final): `cargo run` | Let's put the semicolon back. This time, instead of cargo run, I'll use cargo check. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Install Rust and create `hello_rust`
- Print your name, your city and your favourite number
- Break the code on purpose three different ways, and read each error
- Run `cargo fmt` after messing up the indentation: what happens?
