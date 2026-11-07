fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let s1 = String::from("ferris");
    let result;
    {
        let s2 = String::from("crab");
        result = longest(&s1, &s2);
        println!("inside: {result}");
    }
    // println!("outside: {result}"); // error: s2 does not live long enough
}
