fn main() {
    let y = {
        let x = 3;
        x * x + 1
    };
    println!("y = {y}");

    for n in [7, 10, 15] {
        let size = if n > 9 { "big" } else { "small" };
        let parity = if n % 2 == 0 { "even" } else { "odd" };
        println!("{n} is {size} and {parity}");
    }
}
