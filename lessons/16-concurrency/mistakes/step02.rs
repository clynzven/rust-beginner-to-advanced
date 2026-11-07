use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("sum = {}", data.iter().sum::<i32>());
    });
    handle.join().unwrap();
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling concurrency v0.1.0 (~/rust-course/concurrency)
// error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
//  --> src/main.rs:5:32
//   |
// 5 |     let handle = thread::spawn(|| {
//   |                                ^^ may outlive borrowed value `data`
// 6 |         println!("sum = {}", data.iter().sum::<i32>());
//   |                              ---- `data` is borrowed here
//   |
// note: function requires argument type to outlive `'static`
//  --> src/main.rs:5:18
//   |
// 5 |       let handle = thread::spawn(|| {
//   |  __________________^
// 6 | |         println!("sum = {}", data.iter().sum::<i32>());
// 7 | |     });
//   | |______^
// help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
//   |
// 5 |     let handle = thread::spawn(move || {
//   |                                ++++
//
// For more information about this error, try `rustc --explain E0373`.
// error: could not compile `concurrency` (bin "concurrency") due to 1 previous error
