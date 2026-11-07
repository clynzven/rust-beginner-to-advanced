fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling lifetimes v0.1.0 (~/rust-course/lifetimes)
// error[E0597]: `x` does not live long enough
//  --> src/main.rs:5:13
//   |
// 4 |         let x = 5;
//   |             - binding `x` declared here
// 5 |         r = &x;
//   |             ^^ borrowed value does not live long enough
// 6 |     }
//   |     - `x` dropped here while still borrowed
// 7 |     println!("r: {r}");
//   |                   - borrow later used here
//
// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `lifetimes` (bin "lifetimes") due to 1 previous error
