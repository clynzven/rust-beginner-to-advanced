#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // borrows self mutably: changes the rectangle
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // takes ownership of self: the rectangle is consumed
    fn into_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

fn main() {
    let mut r = Rectangle { width: 3, height: 4 };
    r.scale(2);
    println!("{r:?}, area {}", r.area());
    let (w, h) = r.into_tuple();
    println!("w={w} h={h}");
}
