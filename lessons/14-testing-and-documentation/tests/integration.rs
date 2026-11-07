use temperature::{c_to_f, parse_celsius};

#[test]
fn body_temperature_round_trip() {
    let c = parse_celsius("98.6F").unwrap();
    assert!((c_to_f(c) - 98.6).abs() < 1e-9);
}
