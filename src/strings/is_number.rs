//! Is Number?

/// Determines if a given string can be interpreted as a decimal number. Valid characters include
/// digits [0-9], a positive or negative sign '+' / '-', a decimal point '.', and an exponent 'e'.
/// Fractional exponents are invalid. Leading and trailing whitespaces are ignored.
///
/// # Examples
///
///     # use rustic::strings::is_number::is_number;
///     assert_eq!(is_number(" 123 "), true);
///     assert_eq!(is_number("3.14"), true);
///     assert_eq!(is_number("abc"), false);
///     assert_eq!(is_number("--6"), false);
///
///     assert_eq!(is_number("1e-3"), true);
///     assert_eq!(is_number("-2.73e-2"), true);
///     assert_eq!(is_number("3e"), false);
///     assert_eq!(is_number("4e2.5"), false);
///
pub fn is_number(s: &str) -> bool {
    let mut valid_decimal = false;
    let mut valid_exponent = true;

    for (i, part) in s.trim().split('e').enumerate() {
        match i {
            0 => valid_decimal = is_decimal(part),
            1 => valid_exponent = is_integer(part),
            _ => return false,
        }
    }

    valid_decimal && valid_exponent
}


fn is_decimal(s: &str) -> bool {
    match s.parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}


fn is_integer(s: &str) -> bool {
    match s.parse::<i64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
