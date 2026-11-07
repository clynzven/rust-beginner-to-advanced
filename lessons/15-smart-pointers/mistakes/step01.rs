#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Nil));
    println!("{list:?}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling smart_pointers v0.1.0 (~/rust-course/smart_pointers)
// error[E0072]: recursive type `List` has infinite size
//  --> src/main.rs:2:1
//   |
// 2 | enum List {
//   | ^^^^^^^^^
// 3 |     Cons(i32, List),
//   |               ---- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
//   |
// 3 |     Cons(i32, Box<List>),
//   |               ++++    +
//
// For more information about this error, try `rustc --explain E0072`.
// error: could not compile `smart_pointers` (bin "smart_pointers") due to 1 previous error
