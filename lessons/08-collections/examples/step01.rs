fn main() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(72);
    scores.push(95);
    scores.push(88);

    let more = vec![60, 100];
    scores.extend(more);

    println!("{scores:?} len={}", scores.len());
    println!("first = {}", scores[0]);
    println!("get(10) = {:?}", scores.get(10));
    println!("popped = {:?}", scores.pop());

    scores.sort();
    println!("sorted = {scores:?}");
    println!("contains 95? {}", scores.contains(&95));
}
