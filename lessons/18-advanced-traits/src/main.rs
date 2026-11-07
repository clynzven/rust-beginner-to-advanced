use std::fmt;

// We can't implement Display for Vec<String> directly (both are foreign),
// so we wrap it in a local "newtype".
struct List(Vec<String>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(" | "))
    }
}

trait Greet {
    fn hello(&self) -> String;
}

// supertrait: anything that is Greet + Display can be Loud
trait Loud: Greet + fmt::Display {
    fn shout(&self) -> String {
        format!("{} {}!", self.hello().to_uppercase(), self)
    }
}

impl Greet for List {
    fn hello(&self) -> String { "hello".into() }
}
impl Loud for List {}

fn main() {
    let l = List(vec!["tea".into(), "coffee".into()]);
    println!("{l}");
    println!("{}", l.shout());
}
