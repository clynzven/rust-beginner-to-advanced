fn main() {
    // loop: repeat forever until `break`, which can return a value
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count * 10;
        }
    };
    println!("loop result: {result}");

    // while: repeat while a condition is true
    let mut n = 3;
    while n > 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("Liftoff!");

    // for: the most common loop, over a range or a collection
    for i in 1..=3 {
        print!("{i} ");
    }
    println!();
    for day in ["Mon", "Tue", "Wed"].iter().rev() {
        print!("{day} ");
    }
    println!();
}
