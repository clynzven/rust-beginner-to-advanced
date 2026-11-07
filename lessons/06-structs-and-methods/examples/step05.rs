#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated function (no self): called as Rectangle::square(5)
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    // method: borrows self immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let big = Rectangle { width: 30, height: 50 };
    let small = Rectangle::square(10);
    println!("big area = {}", big.area());
    println!("small = {small:?}, area = {}", small.area());
    println!("big can hold small? {}", big.can_hold(&small));
    println!("small can hold big? {}", small.can_hold(&big));
}
