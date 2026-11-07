fn find_user(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Ana"),
        2 => Some("Ben"),
        _ => None,
    }
}

fn main() {
    for id in [1, 2, 3] {
        match find_user(id) {
            Some(name) => println!("user {id}: {name}"),
            None => println!("user {id}: not found"),
        }
    }

    let name = find_user(3).unwrap_or("guest");
    println!("welcome, {name}");

    let lengths: Vec<Option<usize>> = [1, 5].iter().map(|&id| find_user(id).map(|n| n.len())).collect();
    println!("{lengths:?}");
}
