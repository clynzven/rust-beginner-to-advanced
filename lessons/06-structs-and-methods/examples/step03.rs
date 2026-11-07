#[derive(Debug, Clone)]
#[allow(dead_code)] // we only read the fields through Debug
struct User {
    name: String,
    email: String,
    active: bool,
    logins: u32,
}

fn new_user(name: &str, email: &str) -> User {
    User {
        name: name.to_string(),
        email: email.to_string(),
        active: true,
        logins: 0,
    }
}

fn main() {
    let ana = new_user("Ana", "ana@example.com");
    let ben = User {
        name: String::from("Ben"),
        email: String::from("ben@example.com"),
        ..ana.clone()
    };
    println!("{ana:?}");
    println!("{ben:#?}");
}
