fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{s}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s is a reference, so nothing is freed here
