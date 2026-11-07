use crate::pricing;

#[derive(Debug, Default)]
pub struct Cart {
    items: Vec<(String, u64)>,   // private field
}

impl Cart {
    pub fn add(&mut self, name: &str, cents: u64) {
        self.items.push((name.to_string(), cents));
    }

    pub fn total(&self) -> u64 {
        let subtotal: u64 = self.items.iter().map(|(_, c)| c).sum();
        pricing::with_tax(subtotal)
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}
