# Lesson 8: Collections: Vec, String and HashMap

*Part 2 · Intermediate* · Video 8 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Vec, String and HashMap: the three collections you'll use every day.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | A Vec stores values of one type, side by side on the heap, and it can grow. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | There are three ways to loop over a Vec, matching our three ways of taking a value. |
| 3 | `mistakes/step03.rs` (does not compile, on purpose) | Here's a classic surprise. We take a reference to the first element, push a new value, and then print the first element. |
| 4 | `examples/step04.rs`: `cargo run --example step04` | Strings. Push adds one character, push str adds a string slice. |
| 5 | `examples/step05.rs`: `cargo run --example step05` | A HashMap stores key-value pairs. We have to bring it into scope with a use line, since it's not in the prelude. |
| 6 | `src/main.rs` (final): `cargo run` | A few more collections worth knowing. BTreeMap is a map that keeps its keys sorted, so printing it gives a predictable order. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Read a sentence and print each letter's frequency, sorted (use `BTreeMap<char, usize>`)
- Write `fn median(v: &mut Vec<f64>) -> Option<f64>`
- Group words by their first letter: `HashMap<char, Vec<String>>`
- Remove duplicates from a `Vec` while keeping order (hint: a `HashSet` of seen values)
