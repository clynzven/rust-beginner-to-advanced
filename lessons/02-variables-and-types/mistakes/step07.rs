fn main() {
    let person: (&str, u32, f64) = ("Ana", 34, 1.62);
    let (name, age, height) = person;
    println!("{name} is {age} and {height} m tall");
    println!("first field: {}", person.0);

    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let zeros = [0; 4];
    println!("{} days, first {}, zeros {:?}", days.len(), days[0], zeros);

    let input = "7";
    let index: usize = input.parse().unwrap();
    println!("day {index}: {}", days[index]);
}

// ----------------------------------------------------------------
// This version does NOT compile, on purpose. The compiler said:
//
// Compiling variables v0.1.0 (~/rust-course/variables)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
//      Running `target/debug/variables`
// Ana is 34 and 1.62 m tall
// first field: Ana
// 5 days, first Mon, zeros [0, 0, 0, 0]
//
// thread 'main' (905) panicked at src/main.rs:13:33:
// index out of bounds: the len is 5 but the index is 7
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
