fn main() {
    for input in ["42", "-7", "abc", ""] {
        match input.parse::<i32>() {
            Ok(n) => println!("{input:>5} -> Ok({n})"),
            Err(e) => println!("{input:>5} -> Err: {e}"),
        }
    }

    let n: i32 = "12".parse().unwrap();           // panics on Err
    let m: i32 = "x".parse().unwrap_or(0);        // default instead
    println!("n={n} m={m}");
}
