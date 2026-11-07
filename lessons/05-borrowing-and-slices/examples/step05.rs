fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}"); // last use of r1 and r2

    let r3 = &mut s;           // fine: no other borrows are alive
    r3.push_str(" world");
    println!("{r3}");
}
