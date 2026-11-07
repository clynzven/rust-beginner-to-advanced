use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    id: u32,
    customer: String,
    items: Vec<Item>,
    #[serde(default)]
    paid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    name: String,
    #[serde(rename = "qty")]
    quantity: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let json = r#"{ "id": 7, "customer": "Ana",
                    "items": [ { "name": "tea", "qty": 2 } ] }"#;
    let order: Order = serde_json::from_str(json)?;
    println!("{order:?}");

    let back = serde_json::to_string_pretty(&order)?;
    println!("{back}");

    let bad: Result<Order, _> = serde_json::from_str(r#"{ "id": "seven" }"#);
    println!("error: {}", bad.unwrap_err());
    Ok(())
}
