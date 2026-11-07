use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

type Users = Arc<Mutex<HashSet<String>>>;

#[derive(Clone, Debug)]
struct Msg {
    from: SocketAddr,
    text: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("chat server on {}", listener.local_addr()?);

    let (tx, _) = broadcast::channel::<Msg>(100);
    let users: Users = Arc::new(Mutex::new(HashSet::new()));

    loop {
        let (socket, addr) = listener.accept().await?;
        let tx = tx.clone();
        let users = Arc::clone(&users);
        tokio::spawn(async move {
            if let Err(e) = handle(socket, addr, tx, users).await {
                eprintln!("{addr}: {e}");
            }
        });
    }
}

async fn handle(socket: TcpStream, addr: SocketAddr, tx: broadcast::Sender<Msg>, users: Users)
    -> std::io::Result<()>
{
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    writer.write_all(b"Welcome! What's your name?\n").await?;
    let Some(name) = lines.next_line().await? else { return Ok(()) };
    let name = name.trim().to_string();

    if !users.lock().unwrap().insert(name.clone()) {
        writer.write_all(b"Sorry, that name is taken.\n").await?;
        return Ok(());
    }
    let mut rx = tx.subscribe();
    let _ = tx.send(Msg { from: addr, text: format!("* {name} joined") });
    writer.write_all(format!("Hi {name}! Commands: /who, /quit\n").as_bytes()).await?;

    loop {
        tokio::select! {
            line = lines.next_line() => {
                let Some(line) = line? else { break };          // client disconnected
                match line.trim() {
                    "" => {}
                    "/quit" => break,
                    "/who" => {
                        let mut online: Vec<String> = users.lock().unwrap().iter().cloned().collect();
                        online.sort();
                        writer.write_all(format!("Online: {}\n", online.join(", ")).as_bytes()).await?;
                    }
                    text => {
                        let _ = tx.send(Msg { from: addr, text: format!("{name}: {text}") });
                    }
                }
            }
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if msg.from != addr {
                        writer.write_all(format!("{}\n", msg.text).as_bytes()).await?;
                    }
                }
            }
        }
    }

    users.lock().unwrap().remove(&name);
    let _ = tx.send(Msg { from: addr, text: format!("* {name} left") });
    Ok(())
}
