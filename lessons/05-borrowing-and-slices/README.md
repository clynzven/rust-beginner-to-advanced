# Lesson 5: Borrowing and Slices

*Part 1 · Beginner* · Video 5 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

References let you use a value without taking ownership, and the borrow checker keeps them safe.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | Here's borrowing. Calculate length takes an &String, a reference to a String. |
| 2 | `mistakes/step02.rs` (does not compile, on purpose) | Now, can we change something through a reference? |
| 3 | `examples/step03.rs`: `cargo run --example step03` | To change a borrowed value, three things must line up. |
| 4 | `mistakes/step04.rs` (does not compile, on purpose) | Now the first rule. What if we take two mutable references to the same String at the same time, and use both? |
| 5 | `examples/step05.rs`: `cargo run --example step05` | So how do we fix it? Borrows only last until their last use. |
| 6 | `mistakes/step06.rs` (does not compile, on purpose) | Now the second rule: references must always be valid. |
| 7 | `src/main.rs` (final): `cargo run` | Now slices. A slice is a reference to part of a collection. |
| 8 | `mistakes/step08.rs` (does not compile, on purpose) | Here's why slices are safer than indexes. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Write `fn longest_word(s: &str) -> &str`
- Write `fn double_all(v: &mut Vec<i32>)` that doubles every number
- Write `fn sum(slice: &[i32]) -> i32` and call it with an array and part of an array
- Fix the `clear()` example without removing the call to `clear`
