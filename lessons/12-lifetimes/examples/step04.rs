#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    fn first_sentence(&self) -> &'a str {
        self.text.split('.').next().unwrap_or("")
    }
}

fn main() {
    let book = String::from("Call me Ishmael. Some years ago, never mind how long.");
    let e = Excerpt { text: &book[..30] };
    println!("{e:?}");
    println!("first: {}", e.first_sentence());
}
