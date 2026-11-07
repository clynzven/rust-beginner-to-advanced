use std::time::Duration;

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let a = tokio::spawn(async {
        std::thread::sleep(Duration::from_millis(200)); // blocks the thread!
    });
    let b = tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(200)); // fine: dedicated thread
        "heavy work done"
    });
    a.await.unwrap();
    println!("{} after ~{} ms", b.await.unwrap(), start.elapsed().as_millis() / 100 * 100);
}
