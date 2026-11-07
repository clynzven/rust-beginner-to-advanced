use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  worker: step {i}");
            thread::sleep(Duration::from_millis(10));
        }
        42
    });

    println!("main: waiting for the worker");
    let answer = handle.join().unwrap();
    println!("main: worker returned {answer}");
}
