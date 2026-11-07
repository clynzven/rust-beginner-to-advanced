use rand::seq::IndexedRandom;
use shop::cart::Cart;

fn main() {
    let menu = [("coffee", 550), ("muffin", 450), ("scone", 400), ("tea", 380)];
    let mut rng = rand::rng();
    let mut cart = Cart::default();
    for _ in 0..3 {
        let (name, price) = menu.choose(&mut rng).unwrap();
        cart.add(name, *price);
    }
    println!("{} random items, total {} cents", cart.count(), cart.total());
    println!("table number: {}", rand::random_range(1..=20));
}
