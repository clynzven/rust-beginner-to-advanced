fn main() {
    let mut s = String::from("Kia");
    s.push(' ');
    s.push_str("ora");
    let greeting = s + ", world";          // s is moved here
    let full = format!("{greeting}! ({} bytes)", greeting.len());
    println!("{full}");

    let word = "héllo";
    println!("bytes: {}, chars: {}", word.len(), word.chars().count());
    for c in word.chars().rev() {
        print!("{c}");
    }
    println!();
    println!("{}", word.replace("é", "e").to_uppercase());
}
