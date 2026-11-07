trait Describe {
    fn name(&self) -> String;

    fn describe(&self) -> String {          // default method
        format!("This is {}.", self.name())
    }
}

struct Dog;
struct Robot { model: u32 }

impl Describe for Dog {
    fn name(&self) -> String {
        "a dog".into()
    }
}

impl Describe for Robot {
    fn name(&self) -> String {
        format!("robot #{}", self.model)
    }
    fn describe(&self) -> String {          // override the default
        format!("BEEP. I AM {}.", self.name().to_uppercase())
    }
}

fn main() {
    println!("{}", Dog.describe());
    println!("{}", Robot { model: 42 }.describe());
}
