use std::rc::Rc;

#[derive(Debug)]
struct Config {
    name: String,
}

struct Service {
    label: &'static str,
    config: Rc<Config>,
}

fn main() {
    let config = Rc::new(Config { name: "production".into() });
    println!("count after new = {}", Rc::strong_count(&config));

    let api = Service { label: "api", config: Rc::clone(&config) };
    let worker = Service { label: "worker", config: Rc::clone(&config) };
    println!("count with two services = {}", Rc::strong_count(&config));

    for s in [&api, &worker] {
        println!("{} uses {}", s.label, s.config.name);
    }
    drop(worker);
    println!("count after drop = {}", Rc::strong_count(&config));
}
