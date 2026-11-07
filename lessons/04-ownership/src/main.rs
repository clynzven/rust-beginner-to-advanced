fn main() {
    let name = String::from("Ferris");
    let name = shout(name);
    println!("Got back: {name}");
}

fn shout(s: String) -> String {
    s.to_uppercase() + "!"
}
