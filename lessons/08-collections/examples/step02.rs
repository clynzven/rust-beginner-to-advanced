fn main() {
    let mut names = vec![String::from("ana"), String::from("ben")];

    for name in &names {           // borrow each element
        print!("{name} ");
    }
    println!();

    for name in &mut names {       // change each element in place
        *name = name.to_uppercase();
    }
    println!("{names:?}");

    for name in names {            // move each element out: names is consumed
        print!("[{name}] ");
    }
    println!();
}
