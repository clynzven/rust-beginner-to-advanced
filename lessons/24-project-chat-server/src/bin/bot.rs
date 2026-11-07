// A scripted test client: bot <name> <delay_ms> "msg1|msg2|..."
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (name, delay, script) = (&args[1], args[2].parse::<u64>().unwrap(), &args[3]);

    sleep(Duration::from_millis(delay)).await;
    let (reader, mut writer) = TcpStream::connect("127.0.0.1:7878").await?.into_split();
    let mut lines = BufReader::new(reader).lines();

    let printer = tokio::spawn(async move {
        let mut seen = vec![];
        while let Ok(Some(line)) = lines.next_line().await {
            seen.push(line);
        }
        seen
    });

    writer.write_all(format!("{name}\n").as_bytes()).await?;
    for msg in script.split('|') {
        sleep(Duration::from_millis(300)).await;
        writer.write_all(format!("{msg}\n").as_bytes()).await?;
    }
    sleep(Duration::from_millis(300)).await;
    drop(writer);
    for line in printer.await.unwrap() {
        println!("  [{name} sees] {line}");
    }
    Ok(())
}
