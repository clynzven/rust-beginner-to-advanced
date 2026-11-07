use std::ops::{Add, Mul, Neg, Index};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 { Vec2 { x: self.x + o.x, y: self.y + o.y } }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, k: f64) -> Vec2 { Vec2 { x: self.x * k, y: self.y * k } }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2 { x: -self.x, y: -self.y } }
}

struct Path(Vec<Vec2>);

impl Index<usize> for Path {
    type Output = Vec2;
    fn index(&self, i: usize) -> &Vec2 { &self.0[i] }
}

fn main() {
    let pos = Vec2 { x: 1.0, y: 2.0 };
    let velocity = Vec2 { x: 0.5, y: -1.0 };
    let next = pos + velocity * 2.0;
    println!("{next:?}");
    println!("{:?}", -next);
    let path = Path(vec![pos, next]);
    println!("path[1] = {:?}", path[1]);
}
