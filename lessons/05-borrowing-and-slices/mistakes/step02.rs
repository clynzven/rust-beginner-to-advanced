fn main() {
    let s = String::from("hello");
    add_exclamation(&s);
    println!("{s}");
}

fn add_exclamation(s: &String) {
    s.push_str("!");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling borrowing v0.1.0 (~/rust-course/borrowing)
// error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
//  --> src/main.rs:8:5
//   |
// 8 |     s.push_str("!");
//   |     ^ `s` is a `&` reference, so it cannot be borrowed as mutable
//   |
// help: consider changing this to be a mutable reference
//   |
// 7 | fn add_exclamation(s: &mut String) {
//   |                        +++
//
// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
