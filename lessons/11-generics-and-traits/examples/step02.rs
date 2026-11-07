#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    fn sum(&self) -> T {
        self.x + self.y
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let a = Point { x: 3, y: 4 };
    let b = Point { x: 3.0, y: 4.0 };
    println!("{a:?} sum={}", a.sum());
    println!("{b:?} sum={} distance={}", b.sum(), b.distance_from_origin());
}
