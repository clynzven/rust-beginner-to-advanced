macro_rules! my_vec {
    ( $( $item:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($item); )*
        v
    }};
}

macro_rules! hashmap {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut m = std::collections::BTreeMap::new();
        $( m.insert($k, $v); )*
        m
    }};
}

macro_rules! max {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {{
        let a = $x;
        let b = max!($($rest),+);
        if a > b { a } else { b }
    }};
}

fn main() {
    let v = my_vec![1, 2, 3, 4];
    println!("{v:?}");
    let ages = hashmap! { "ana" => 34, "ben" => 28, };
    println!("{ages:?}");
    println!("max = {}", max!(3, 9, 4, 7));
}
