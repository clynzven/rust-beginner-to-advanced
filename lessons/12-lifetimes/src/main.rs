fn main() {
    let greeting: &'static str = "lives for the whole program";
    println!("{greeting}");

    // Most of the time, owning the data is simpler than borrowing it:
    struct OwnedExcerpt {
        text: String,
    }
    let e = {
        let book = String::from("Call me Ishmael.");
        OwnedExcerpt { text: book.clone() }
    };
    println!("owned: {}", e.text);
}
