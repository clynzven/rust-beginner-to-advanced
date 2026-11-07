use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn fetch(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} ({ms} ms)")
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (a, b, c) = tokio::join!(
        fetch("users", 300),
        fetch("orders", 200),
        fetch("prices", 100),
    );
    println!("{a}, {b}, {c}");
    println!("concurrently: {} ms", start.elapsed().as_millis() / 100 * 100);
}
