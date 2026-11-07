const MAX_POINTS: u32 = 100_000;

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner x: {x}");
    }
    println!("Outer x: {x}");
    println!("Max points: {MAX_POINTS}");
}
