fn main() {
    // A comment: the compiler ignores everything after two slashes.
    let name = "Rustacean";
    let year = 2026;

    println!("Hello, {name}!");
    println!("Welcome to Rust in {year}.");
    println!("{} + {} = {}", 2, 3, 2 + 3)
    println!("Done!");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling hello_rust v0.1.0 (~/rust-course/hello_rust)
// error: expected `;`, found `println`
//  --> src/main.rs:8:42
//   |
// 8 |     println!("{} + {} = {}", 2, 3, 2 + 3)
//   |                                          ^ help: add `;` here
// 9 |     println!("Done!");
//   |     ------- unexpected token
//
// error: could not compile `hello_rust` (bin "hello_rust") due to 1 previous error
