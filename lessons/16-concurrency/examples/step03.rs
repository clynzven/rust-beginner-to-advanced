use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for id in 1..=3 {
        let tx = tx.clone();
        thread::spawn(move || {
            let result = id * id;
            tx.send(format!("worker {id} computed {result}")).unwrap();
        });
    }
    drop(tx); // close our copy so the loop below can end

    let mut messages: Vec<String> = rx.iter().collect();
    messages.sort();
    for m in &messages {
        println!("{m}");
    }
}
