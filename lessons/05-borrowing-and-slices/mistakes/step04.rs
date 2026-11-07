fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push_str(" world");
    println!("{r1} {r2}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling borrowing v0.1.0 (~/rust-course/borrowing)
// error[E0499]: cannot borrow `s` as mutable more than once at a time
//  --> src/main.rs:4:14
//   |
// 3 |     let r1 = &mut s;
//   |              ------ first mutable borrow occurs here
// 4 |     let r2 = &mut s;
//   |              ^^^^^^ second mutable borrow occurs here
// 5 |     r1.push_str(" world");
//   |     -- first borrow later used here
//
// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
