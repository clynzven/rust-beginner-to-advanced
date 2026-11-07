macro_rules! newtype {
    ($name:ident, $inner:ty, $unit:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        struct $name($inner);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {}", self.0, $unit)
            }
        }
    };
}

newtype!(Meters, f64, "m");
newtype!(Seconds, f64, "s");
newtype!(Kilograms, u32, "kg");

fn main() {
    let d = Meters(100.0);
    let t = Seconds(9.58);
    println!("{d} in {t}: {:.2} m/s", d.0 / t.0);
    println!("{}", Kilograms(72));
}
