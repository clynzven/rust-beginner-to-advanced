fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!");
    println!("{s2}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling ownership v0.1.0 (~/rust-course/ownership)
// error[E0382]: borrow of moved value: `s1`
//  --> src/main.rs:4:16
//   |
// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 |     println!("{s1}, world!");
//   |                ^^ value borrowed here after move
//   |
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     let s2 = s1.clone();
//   |                ++++++++
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` (bin "ownership") due to 1 previous error
