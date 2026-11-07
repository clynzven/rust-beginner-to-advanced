# Lesson 18: Advanced Traits and Trait Objects

*Part 3 · Advanced* · Video 18 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Trait objects with dyn, associated types, operator overloading, and the newtype pattern.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `examples/step01.rs`: `cargo run --example step01` | A Vec can only hold one type. But what if we want a list of different shapes? |
| 2 | `examples/step02.rs`: `cargo run --example step02` | An associated type is a placeholder type inside a trait, that each implementation fills in. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Operator overloading in Rust is just implementing traits from std ops. |
| 4 | `src/main.rs` (final): `cargo run` | Two last patterns. The orphan rule says you can only implement a trait for a type if either the trait or the type is defined in your crate. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Build a `Vec<Box<dyn Plugin>>` where each plugin transforms a `String`
- Implement `Add`, `Sub` and `AddAssign` for `Money` from lesson 11
- Write a `trait Graph { type Node; fn neighbours(&self, n: &Self::Node) -> Vec<Self::Node>; }`
- Wrap `HashMap<String, u32>` in a newtype and implement `Display` for it
