fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    println!("{}", longest("ferris", "crab"));
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling lifetimes v0.1.0 (~/rust-course/lifetimes)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:1:33
//   |
// 1 | fn longest(a: &str, b: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
// help: consider introducing a named lifetime parameter
//   |
// 1 | fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++
//
// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `lifetimes` (bin "lifetimes") due to 1 previous error
