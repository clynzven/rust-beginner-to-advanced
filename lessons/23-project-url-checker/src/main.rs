use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

/// Check whether a list of URLs is up.
#[derive(Parser)]
struct Args {
    /// File with one URL per line
    file: String,
    /// Maximum requests in flight at once
    #[arg(short, long, default_value_t = 10)]
    concurrency: usize,
    /// Timeout per request, in milliseconds
    #[arg(short, long, default_value_t = 1000)]
    timeout_ms: u64,
}

#[derive(Debug)]
enum Outcome {
    Up(u16),
    Down(u16),
    Error(String),
}

struct Report {
    index: usize,
    url: String,
    outcome: Outcome,
    millis: u128,
}

async fn check(client: &reqwest::Client, url: &str) -> Outcome {
    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => Outcome::Up(resp.status().as_u16()),
        Ok(resp) => Outcome::Down(resp.status().as_u16()),
        Err(e) if e.is_timeout() => Outcome::Error("timed out".into()),
        Err(e) if e.is_connect() => Outcome::Error("connection refused".into()),
        Err(e) if e.is_builder() => Outcome::Error("invalid URL".into()),
        Err(e) => Outcome::Error(e.to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let text = std::fs::read_to_string(&args.file)?;
    let urls: Vec<String> = text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(args.timeout_ms))
        .build()?;
    let limit = Arc::new(Semaphore::new(args.concurrency));
    let started = Instant::now();

    let mut tasks = JoinSet::new();
    for (index, url) in urls.into_iter().enumerate() {
        let client = client.clone();           // cheap: shares one connection pool
        let limit = Arc::clone(&limit);
        tasks.spawn(async move {
            let _permit = limit.acquire_owned().await.unwrap();   // wait for a free slot
            let t = Instant::now();
            let outcome = check(&client, &url).await;
            Report { index, url, outcome, millis: t.elapsed().as_millis() }
        });
    }

    let mut reports = Vec::new();
    while let Some(r) = tasks.join_next().await {
        reports.push(r?);
    }
    reports.sort_by_key(|r| r.index);

    let (mut up, mut down, mut err) = (0, 0, 0);
    for r in &reports {
        let (verdict, detail) = match &r.outcome {
            Outcome::Up(code) => { up += 1; ("UP", code.to_string()) }
            Outcome::Down(code) => { down += 1; ("DOWN", code.to_string()) }
            Outcome::Error(msg) => { err += 1; ("ERROR", msg.clone()) }
        };
        let ms = (r.millis + 50) / 100 * 100;  // round to 100 ms for a stable demo
        println!("{verdict:<6} {detail:<19} ~{ms:>4} ms  {}", r.url);
    }
    let total = (started.elapsed().as_millis() + 50) / 100 * 100;
    println!("\n{} checked: {up} up, {down} down, {err} errors in ~{total} ms (concurrency {})",
             reports.len(), args.concurrency);
    Ok(())
}
