use std::collections::HashMap;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the end";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("'the' appears {} times", counts["the"]);
    println!("'cat' -> {:?}", counts.get("cat"));

    let mut sorted: Vec<(&str, u32)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    for (word, n) in sorted.iter().take(3) {
        println!("{word:>6}: {n}");
    }
}
