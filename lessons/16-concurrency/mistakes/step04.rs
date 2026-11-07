use std::rc::Rc;
use std::thread;

fn main() {
    let counter = Rc::new(0);
    let c = Rc::clone(&counter);
    thread::spawn(move || {
        println!("{c}");
    }).join().unwrap();
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling concurrency v0.1.0 (~/rust-course/concurrency)
// error[E0277]: `Rc<i32>` cannot be sent between threads safely
//  --> src/main.rs:7:19
//   |
// 7 |       thread::spawn(move || {
//   |       ------------- ^------
//   |       |             |
//   |  _____|_____________within this `{closure@src/main.rs:7:19: 7:26}`
//   | |     |
//   | |     required by a bound introduced by this call
// 8 | |         println!("{c}");
// 9 | |     }).join().unwrap();
//   | |_____^ `Rc<i32>` cannot be sent between threads safely
//   |
//   = help: within `{closure@src/main.rs:7:19: 7:26}`, the trait `Send` is not implemented for `Rc<i32>`
// note: required because it's used within this closure
//  --> src/main.rs:7:19
//   |
// 7 |     thread::spawn(move || {
//   |                   ^^^^^^^
// note: required by a bound in `spawn`
//  --> /rustc/2d8144b7880597b6e6d3dfd63a9a9efae3f533d3/library/std/src/thread/functions.rs:125:0
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `concurrency` (bin "concurrency") due to 1 previous error
