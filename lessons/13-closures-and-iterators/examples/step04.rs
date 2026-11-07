use std::collections::HashMap;

#[derive(Debug)]
struct Sale { region: &'static str, item: &'static str, amount: u32 }

fn main() {
    let sales = vec![
        Sale { region: "north", item: "tea", amount: 120 },
        Sale { region: "south", item: "coffee", amount: 300 },
        Sale { region: "north", item: "coffee", amount: 250 },
        Sale { region: "south", item: "tea", amount: 80 },
        Sale { region: "east", item: "tea", amount: 150 },
    ];

    let mut by_region: HashMap<&str, u32> = HashMap::new();
    for s in &sales {
        *by_region.entry(s.region).or_default() += s.amount;
    }
    let mut regions: Vec<_> = by_region.into_iter().collect();
    regions.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{regions:?}");

    let best = sales.iter().max_by_key(|s| s.amount).unwrap();
    println!("best sale: {} {} {}", best.region, best.item, best.amount);

    let tea: Vec<String> = sales.iter()
        .filter(|s| s.item == "tea")
        .enumerate()
        .map(|(i, s)| format!("{}. {} ({})", i + 1, s.region, s.amount))
        .collect();
    println!("{}", tea.join(", "));

    let names = ["a", "b", "c"];
    let zipped: Vec<_> = names.iter().zip(1..).collect();
    println!("{zipped:?}");
}
