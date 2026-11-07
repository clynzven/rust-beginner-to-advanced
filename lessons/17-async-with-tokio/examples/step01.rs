use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;     // pretend network call
    format!("{name} ({ms} ms)")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let a = fetch("users", 300).await;
    let b = fetch("orders", 200).await;
    let c = fetch("prices", 100).await;
    println!("{a}, {b}, {c}");
    println!("one after another: {} ms", start.elapsed().as_millis() / 100 * 100);
}
