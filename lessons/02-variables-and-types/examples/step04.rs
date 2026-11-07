fn main() {
    let a: i32 = -42;
    let b: u8 = 255;
    let pi = 3.14159;          // f64 by default
    let ready: bool = true;
    let letter: char = 'R';

    println!("a={a} b={b} pi={pi} ready={ready} letter={letter}");
    println!("7 / 2 = {}", 7 / 2);
    println!("7 % 2 = {}", 7 % 2);
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);

    let total = a + b as i32;
    println!("a + b = {total}");
}
