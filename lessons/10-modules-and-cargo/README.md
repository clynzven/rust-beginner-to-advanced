# Lesson 10: Modules, Crates and Cargo

*Part 2 · Intermediate* · Video 10 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Organise a growing program into modules and files, control visibility, and use crates from crates.io.

## Run it

`cargo run`, and `cargo test`.

## Exercises

- Add a `discount` module with `pub fn apply(cents: u64, percent: u8) -> u64`
- Add `Cart::total_with_discount` that uses it
- Try to access `cart.items` from `main` and read the error
- Run `cargo doc --open` and browse your own crate's documentation
