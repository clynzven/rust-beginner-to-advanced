fn main() {
    let level: u8 = 255;
    let next = level + 1;
    println!("next level: {next}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling variables v0.1.0 (~/rust-course/variables)
// error: this arithmetic operation will overflow
//  --> src/main.rs:3:16
//   |
// 3 |     let next = level + 1;
//   |                ^^^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
//   |
//   = note: `#[deny(arithmetic_overflow)]` on by default
//
// error: could not compile `variables` (bin "variables") due to 1 previous error
