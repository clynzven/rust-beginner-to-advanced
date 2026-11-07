enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => "quit".to_string(),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Write(text) => format!("write '{text}'"),
        }
    }
}

fn main() {
    println!("{}", Message::Quit.describe());
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling enums v0.1.0 (~/rust-course/enums)
// error[E0004]: non-exhaustive patterns: `&Message::ChangeColor(_, _, _)` not covered
//   --> src/main.rs:10:15
//    |
// 10 |         match self {
//    |               ^^^^ pattern `&Message::ChangeColor(_, _, _)` not covered
//    |
// note: `Message` defined here
//   --> src/main.rs:1:6
//    |
//  1 | enum Message {
//    |      ^^^^^^^
// ...
//  5 |     ChangeColor(u8, u8, u8),
//    |     ----------- not covered
//    = note: the matched value is of type `&Message`
// help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
//    |
// 13 ~             Message::Write(text) => format!("write '{text}'"),
// 14 ~             &Message::ChangeColor(_, _, _) => todo!(),
//    |
//
// For more information about this error, try `rustc --explain E0004`.
// error: could not compile `enums` (bin "enums") due to 1 previous error
