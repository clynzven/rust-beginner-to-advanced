fn main() {
    greet("Clinton");
    let sum = add(2, 3);
    println!("2 + 3 = {sum}");
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
