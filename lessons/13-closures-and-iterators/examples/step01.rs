fn main() {
    let add_one = |x: i32| x + 1;
    let square = |x| x * x;                 // types inferred
    println!("{} {}", add_one(4), square(6));

    let tax_rate = 0.15;
    let with_tax = |price: f64| price * (1.0 + tax_rate);   // captures tax_rate
    println!("{:.2}", with_tax(20.0));

    let mut count = 0;
    let mut tick = || count += 1;           // captures count mutably
    tick();
    tick();
    println!("count = {count}");

    let name = String::from("Ferris");
    let greet = move || format!("Hi, {name}!");  // takes ownership of name
    println!("{}", greet());
}
