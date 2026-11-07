struct Fibonacci { a: u64, b: u64 }

impl Iterator for Fibonacci {
    type Item = u64;                    // associated type
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(r)
    }
}

trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
    fn first(&self) -> Option<&Self::Item> { self.get(0) }
}

struct Stack { items: Vec<String> }

impl Container for Stack {
    type Item = String;
    fn get(&self, i: usize) -> Option<&String> { self.items.get(i) }
}

fn main() {
    let fib: Vec<u64> = Fibonacci { a: 0, b: 1 }.take(10).collect();
    println!("{fib:?}");
    let s = Stack { items: vec!["a".into(), "b".into()] };
    println!("{:?} {:?}", s.first(), s.get(5));
}
