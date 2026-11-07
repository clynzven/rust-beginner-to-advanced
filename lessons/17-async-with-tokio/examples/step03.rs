use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for id in 1..=5u64 {
        set.spawn(async move {
            sleep(Duration::from_millis(60 - id * 10)).await;
            id * 100
        });
    }

    let mut order = vec![];
    while let Some(result) = set.join_next().await {
        order.push(result.unwrap());
    }
    println!("finished in order: {order:?}");

    let handle = tokio::spawn(async { "background task done" });
    println!("{}", handle.await.unwrap());
}
