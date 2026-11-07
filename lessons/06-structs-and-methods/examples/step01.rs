struct User {
    name: String,
    email: String,
    active: bool,
    logins: u32,
}

fn main() {
    let mut user = User {
        name: String::from("Ana"),
        email: String::from("ana@example.com"),
        active: true,
        logins: 1,
    };
    user.logins += 1;
    println!("{} <{}> active={} logins={}", user.name, user.email, user.active, user.logins);
}
