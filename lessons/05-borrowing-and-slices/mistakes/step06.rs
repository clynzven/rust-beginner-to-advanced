fn main() {
    let r = dangle();
    println!("{r}");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling borrowing v0.1.0 (~/rust-course/borrowing)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:6:16
//   |
// 6 | fn dangle() -> &String {
//   |                ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
//   |
// 6 | fn dangle() -> &'static String {
//   |                 +++++++
// help: instead, you are more likely to want to return an owned value
//   |
// 6 - fn dangle() -> &String {
// 6 + fn dangle() -> String {
//   |
//
// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
