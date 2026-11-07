enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => "quit".to_string(),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Write(text) => format!("write '{text}'"),
            Message::ChangeColor(r, g, b) => format!("colour #{r:02x}{g:02x}{b:02x}"),
        }
    }
}

fn main() {
    let messages = [
        Message::Move { x: 3, y: -1 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(226, 115, 58),
        Message::Quit,
    ];
    for m in &messages {
        println!("{}", m.describe());
    }
}
