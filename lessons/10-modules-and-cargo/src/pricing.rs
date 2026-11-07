/// Adds 15% GST to a price in cents.
pub fn with_tax(cents: u64) -> u64 {
    cents + tax(cents)
}

fn tax(cents: u64) -> u64 {
    cents * 15 / 100
}
