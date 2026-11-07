use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Money {
    cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.cents / 100, self.cents % 100)
    }
}

impl From<f64> for Money {
    fn from(dollars: f64) -> Self {
        Money { cents: (dollars * 100.0).round() as i64 }
    }
}

fn main() {
    let price = Money::from(19.99);
    let tip: Money = 3.5.into();
    println!("price {price}, tip {tip}, default {}", Money::default());
    println!("price > tip? {}", price > tip);
    println!("equal? {}", price == Money { cents: 1999 });
    println!("{:?}", price);
}
