struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("  dropping {}", self.0);
    }
}

fn main() {
    let _a = Noisy("a");
    {
        let _b = Noisy("b");
        println!("inner scope ends");
    }
    let c = Noisy("c");
    take(c);
    println!("main ends");
}

fn take(n: Noisy) {
    println!("take() got {}", n.0);
}
