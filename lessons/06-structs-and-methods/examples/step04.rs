#[derive(Debug)]
struct Color(u8, u8, u8);   // tuple struct

#[derive(Debug)]
struct Meters(f64);         // a "newtype" wrapper

#[derive(Debug)]
struct Marker;              // unit struct: no fields

fn main() {
    let orange = Color(226, 115, 58);
    let Color(r, g, b) = orange;
    println!("{orange:?} -> r={r} g={g} b={b}, red channel {}", orange.0);

    let height = Meters(1.62);
    println!("{height:?} is {} cm", height.0 * 100.0);
    println!("{:?}", Marker);
}
