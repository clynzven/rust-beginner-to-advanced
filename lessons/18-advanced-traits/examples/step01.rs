trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;
}

struct Circle { r: f64 }
struct Square { side: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r }
    fn name(&self) -> String { format!("circle r={}", self.r) }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
    fn name(&self) -> String { format!("square side={}", self.side) }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 1.0 }),
        Box::new(Square { side: 3.0 }),
        Box::new(Circle { r: 0.5 }),
    ];
    for s in &shapes {
        println!("{:<16} area {:.2}", s.name(), s.area());
    }
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area {total:.2}");
}
