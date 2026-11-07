fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens_squared: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("{evens_squared:?}");

    let total: i32 = v.iter().sum();
    let big = v.iter().filter(|&&x| x > 7).count();
    let first_over_4 = v.iter().find(|&&x| x > 4);
    let any_negative = v.iter().any(|&x| x < 0);
    println!("sum={total} big={big} first>4={first_over_4:?} negative={any_negative}");

    let lazy = v.iter().map(|x| {
        println!("  visiting {x}");
        x * 10
    });
    println!("nothing has run yet");
    let firsts: Vec<i32> = lazy.take(2).collect();
    println!("{firsts:?}");
}
