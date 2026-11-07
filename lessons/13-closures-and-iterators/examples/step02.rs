fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn main() {
    println!("{}", apply_twice(|x| x + 10, 1));
    let triple = make_multiplier(3);
    println!("{}", apply_twice(&triple, 2));

    let mut words = vec!["pear", "fig", "banana", "kiwi"];
    words.sort_by_key(|w| w.len());
    println!("{words:?}");
}
