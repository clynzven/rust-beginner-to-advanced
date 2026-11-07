use std::cell::RefCell;

struct Logger {
    lines: RefCell<Vec<String>>,
}

impl Logger {
    fn log(&self, msg: &str) {                 // note: &self, not &mut self
        self.lines.borrow_mut().push(msg.to_string());
    }
}

fn main() {
    let logger = Logger { lines: RefCell::new(vec![]) };
    logger.log("started");
    logger.log("loaded 3 users");
    println!("{:?}", logger.lines.borrow());

    let r1 = logger.lines.borrow();
    let r2 = logger.lines.borrow_mut();       // second borrow while r1 is alive
    println!("{} {}", r1.len(), r2.len());
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling smart_pointers v0.1.0 (~/rust-course/smart_pointers)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
//      Running `target/debug/smart_pointers`
// ["started", "loaded 3 users"]
//
// thread 'main' (6046) panicked at src/main.rs:20:27:
// RefCell already borrowed
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
