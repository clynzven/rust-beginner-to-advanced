fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("first is {first}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling collections v0.1.0 (~/rust-course/collections)
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
//  --> src/main.rs:4:5
//   |
// 3 |     let first = &v[0];
//   |                  - immutable borrow occurs here
// 4 |     v.push(4);
//   |     ^^^^^^^^^ mutable borrow occurs here
// 5 |     println!("first is {first}");
//   |                         ----- immutable borrow later used here
//
// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `collections` (bin "collections") due to 1 previous error
