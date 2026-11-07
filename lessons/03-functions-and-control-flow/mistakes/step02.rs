fn main() {
    greet("Clinton");
    let sum = add(2, 3);
    println!("2 + 3 = {sum}");

    let y = {
        let x = 3;
        x * x + 1
    };
    println!("y = {y}");
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b;
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling control_flow v0.1.0 (~/rust-course/control_flow)
// error[E0308]: mismatched types
//   --> src/main.rs:17:27
//    |
// 17 | fn add(a: i32, b: i32) -> i32 {
//    |    ---                    ^^^ expected `i32`, found `()`
//    |    |
//    |    implicitly returns `()` as its body has no tail or `return` expression
// 18 |     a + b;
//    |          - help: remove this semicolon to return this value
//
// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `control_flow` (bin "control_flow") due to 1 previous error
