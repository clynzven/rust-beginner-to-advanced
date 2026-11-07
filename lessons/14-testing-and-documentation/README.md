# Lesson 14: Testing and Documentation

*Part 2 · Intermediate* · Video 14 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Unit tests, integration tests, doc tests and cargo doc: all built in.

## Run it

This lesson is a library: run `cargo test` (unit, integration and doc tests) and `cargo doc --open`.

## Exercises

- Add `pub fn k_to_c(k: f64) -> f64` with a doc test
- Make `parse_celsius` support `K`, and write the test before the code
- Replace the `expect` with a proper `TempError::NotANumber` variant
- Run `cargo test -- --list` and `cargo doc --open`
