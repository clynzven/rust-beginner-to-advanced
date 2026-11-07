fn main() {
    let level: u8 = 255;
    println!("checked:    {:?}", level.checked_add(1));
    println!("wrapping:   {}", level.wrapping_add(1));
    println!("saturating: {}", level.saturating_add(1));
    println!("checked 10: {:?}", 10u8.checked_add(1));
}
