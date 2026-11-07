#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(v, rest) => v + sum(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
    println!("sum = {}", sum(&list));

    let big = Box::new([0u8; 1_000_000]);    // a megabyte on the heap
    println!("boxed array has {} bytes", big.len());
}
