fn parse_age(text: &str) -> Option<u8> {
    text.trim().parse().ok()
}

fn main() {
    // if let: handle one pattern, ignore the rest
    if let Some(age) = parse_age(" 34 ") {
        println!("age is {age}");
    }

    // let else: unwrap or leave early
    let Some(age) = parse_age("abc") else {
        println!("'abc' is not an age");
        return;
    };
    println!("never printed: {age}");
}
