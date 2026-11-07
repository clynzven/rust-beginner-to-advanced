fn main() {
    let sentence = String::from("hello rust world");
    let first = first_word(&sentence);
    println!("first word: {first}");
    println!("slice: {}", &sentence[6..10]);

    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];
    println!("middle: {middle:?}, sum: {}", middle.iter().sum::<i32>());

    println!("from a literal: {}", first_word("good morning"));
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}
