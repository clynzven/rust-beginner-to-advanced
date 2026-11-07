use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let mut n = counter.lock().unwrap();
                *n += 1;
            } // the lock is released here, when the guard is dropped
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final count = {}", *counter.lock().unwrap());
}
