struct User {
    name: String,
    email: String,
    active: bool,
    logins: u32,
}

fn main() {
    let user = User {
        name: String::from("Ana"),
        email: String::from("ana@example.com"),
        active: true,
        logins: 1,
    };
    println!("{user}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling structs v0.1.0 (~/rust-course/structs)
// error[E0277]: `User` doesn't implement `std::fmt::Display`
//   --> src/main.rs:15:15
//    |
// 15 |     println!("{user}");
//    |               ^^^^^^ `User` cannot be formatted with the default formatter
//    |
// help: the trait `std::fmt::Display` is not implemented for `User`
//   --> src/main.rs:1:1
//    |
//  1 | struct User {
//    | ^^^^^^^^^^^
//    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `structs` (bin "structs") due to 1 previous error
