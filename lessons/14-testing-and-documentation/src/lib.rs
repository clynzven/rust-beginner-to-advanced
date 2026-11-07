/// Converts degrees Celsius to Fahrenheit.
///
/// # Examples
///
/// ```
/// use temperature::c_to_f;
/// assert_eq!(c_to_f(100.0), 212.0);
/// ```
pub fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

#[derive(Debug, PartialEq)]
pub enum TempError {
    BelowAbsoluteZero(f64),
}

/// Parses text like `"21.5C"` or `"70F"` into Celsius.
///
/// # Errors
///
/// Returns [`TempError::BelowAbsoluteZero`] for impossible temperatures.
///
/// # Panics
///
/// Panics if the unit is not `C` or `F`.
pub fn parse_celsius(text: &str) -> Result<f64, TempError> {
    let (num, unit) = text.split_at(text.len() - 1);
    let value: f64 = num.parse().expect("not a number");
    let c = match unit {
        "C" => value,
        "F" => f_to_c(value),
        _ => panic!("unknown unit: {unit}"),
    };
    if c < -273.15 { Err(TempError::BelowAbsoluteZero(c)) } else { Ok(c) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_units() {
        assert_eq!(parse_celsius("21.5C"), Ok(21.5));
        assert_eq!(parse_celsius("212F"), Ok(100.0));
    }

    #[test]
    fn rejects_impossible() {
        assert_eq!(parse_celsius("-300C"), Err(TempError::BelowAbsoluteZero(-300.0)));
    }

    #[test]
    #[should_panic(expected = "unknown unit")]
    fn panics_on_bad_unit() {
        parse_celsius("20K").unwrap();
    }

    #[test]
    fn result_style() -> Result<(), TempError> {
        let c = parse_celsius("50F")?;
        assert!(c > 9.9 && c < 10.1);
        Ok(())
    }
}
