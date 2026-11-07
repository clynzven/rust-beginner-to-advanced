# Lesson 15: Smart Pointers

*Part 3 · Advanced* · Video 15 of 24 in the [Rust: Beginner to Advanced playlist](https://www.youtube.com/@ClynzVen/playlists)

Box, Rc, RefCell and Weak: heap allocation, shared ownership and interior mutability.

## Run it

`cargo run`

## Every version from the video

The video builds this program step by step. Each version is here, so you can run any stage.

| Step | File | What it shows |
|---|---|---|
| 1 | `mistakes/step01.rs` (does not compile, on purpose) | Here's a classic linked list, written as an enum. |
| 2 | `examples/step02.rs`: `cargo run --example step02` | Box of T puts a value on the heap, and stores only a pointer on the stack. |
| 3 | `examples/step03.rs`: `cargo run --example step03` | Sometimes a value really has several owners. |
| 4 | `mistakes/step04.rs` (does not compile, on purpose) | RefCell gives interior mutability: you can change data even through a shared reference. |
| 5 | `examples/step05.rs`: `cargo run --example step05` | The common combination is Rc of RefCell: several owners, each of which can change the shared value. |
| 6 | `src/main.rs` (final): `cargo run` | One danger with Rc: if two values point at each other with Rc, their counts never reach zero, and the memory leaks. |

Files in `mistakes/` are the versions the video breaks on purpose to read the compiler error. They are not compiled; the real error is at the bottom of each file. Copy one over `src/main.rs` to see the error yourself.

## Exercises

- Implement `len()` and `push_front()` for the `Box` list
- Build a binary tree with `Option<Box<Node>>` and an `insert` method
- Make a `Counter` with `Rc<RefCell<u32>>` shared by three closures
- Create an `Rc` cycle on purpose, watch `strong_count`, then fix it with `Weak`
