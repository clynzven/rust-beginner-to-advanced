fn main() {
    let mut sentence = String::from("hello rust world");
    let first = first_word(&sentence);
    sentence.clear(); // empties the String
    println!("first word: {first}");
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling borrowing v0.1.0 (~/rust-course/borrowing)
// error[E0502]: cannot borrow `sentence` as mutable because it is also borrowed as immutable
//  --> src/main.rs:4:5
//   |
// 3 |     let first = first_word(&sentence);
//   |                            --------- immutable borrow occurs here
// 4 |     sentence.clear(); // empties the String
//   |     ^^^^^^^^^^^^^^^^ mutable borrow occurs here
// 5 |     println!("first word: {first}");
//   |                            ----- immutable borrow later used here
//
// For more information about this error, try `rustc --explain E0502`.
// error: could not compile `borrowing` (bin "borrowing") due to 1 previous error
