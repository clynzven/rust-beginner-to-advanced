fn main() {
    let name = String::from("Ferris");
    print_name(name);
    println!("Still have {name}?");
}

fn print_name(s: String) {
    println!("Name: {s}");
} // s goes out of scope here, and its memory is freed

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling ownership v0.1.0 (~/rust-course/ownership)
// error[E0382]: borrow of moved value: `name`
//  --> src/main.rs:4:27
//   |
// 2 |     let name = String::from("Ferris");
//   |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
// 3 |     print_name(name);
//   |                ---- value moved here
// 4 |     println!("Still have {name}?");
//   |                           ^^^^ value borrowed here after move
//   |
// note: consider changing this parameter type in function `print_name` to borrow instead if owning the value isn't necessary
//  --> src/main.rs:7:18
//   |
// 7 | fn print_name(s: String) {
//   |    ----------    ^^^^^^ this parameter takes ownership of the value
//   |    |
//   |    in this function
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     print_name(name.clone());
//   |                    ++++++++
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `ownership` (bin "ownership") due to 1 previous error
