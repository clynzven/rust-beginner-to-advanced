macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! greet {
    () => {
        println!("Hello, world!")
    };
    ($name:expr) => {
        println!("Hello, {}!", $name)
    };
}

fn main() {
    println!("{}", square!(7));
    println!("{}", square!(2 + 3));
    greet!();
    greet!("Ferris");
}
