use std::fmt::Display;

trait Describe {
    fn name(&self) -> String;
}

struct Dog;
impl Describe for Dog {
    fn name(&self) -> String { "a dog".into() }
}

impl Describe for i32 {
    fn name(&self) -> String { format!("the number {self}") }
}

// three equivalent ways to write a trait bound
fn introduce(item: &impl Describe) {
    println!("Meet {}", item.name());
}

fn introduce_twice<T: Describe>(a: &T, b: &T) {
    println!("Meet {} and {}", a.name(), b.name());
}

fn show_both<T, U>(a: T, b: U) -> String
where
    T: Display,
    U: Describe,
{
    format!("[{a}] {}", b.name())
}

fn main() {
    introduce(&Dog);
    introduce(&7);
    introduce_twice(&1, &2);
    println!("{}", show_both("label", Dog));
}
