fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}, world!");
    println!("{s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}
