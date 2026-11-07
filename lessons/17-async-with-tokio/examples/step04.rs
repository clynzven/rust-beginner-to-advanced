use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};

async fn slow_query() -> &'static str {
    sleep(Duration::from_secs(5)).await;
    "query result"
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(100), slow_query()).await {
        Ok(v) => println!("got {v}"),
        Err(_) => println!("query timed out after 100 ms"),
    }

    let (tx, mut rx) = mpsc::channel::<String>(8);
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(format!("event {i}")).await.unwrap();
            sleep(Duration::from_millis(30)).await;
        }
    });

    let deadline = sleep(Duration::from_millis(70));
    tokio::pin!(deadline);
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => println!("received {msg}"),
            _ = &mut deadline => {
                println!("deadline reached, stopping");
                break;
            }
        }
    }
}
