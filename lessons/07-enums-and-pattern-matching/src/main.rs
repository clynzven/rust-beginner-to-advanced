fn classify(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        1..=12 => "child",
        13..=19 => "teenager",
        n if n >= 65 => "senior",
        _ => "adult",
    }
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _ => n.to_string(),
    }
}

fn main() {
    for age in [0, 7, 15, 40, 70] {
        println!("{age}: {}", classify(age));
    }
    let line: Vec<String> = (1..=15).map(fizzbuzz).collect();
    println!("{}", line.join(" "));
}
