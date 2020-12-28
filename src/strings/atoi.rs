//! atoi

/// Converts ASCII strings to integers. First, leading whitespace literals ' ' are discarded
/// until the first non-whitespace character is found. Then, as many consecutive numerical 
/// digits as possible are read and interpreted as an integer. Numbers may be prefixed with
/// an optional leading '+' or '-' sign. If no valid integer is found, zero is returned.
///
///
/// # Examples
///
///     # use rustic::strings::atoi::atoi;
///     assert_eq!(atoi("123"), 123);
///     assert_eq!(atoi("  -42"), -42);
///     assert_eq!(atoi("3.14"), 3);
///     assert_eq!(atoi("+300 spartans"), 300);
///     assert_eq!(atoi("spam! 100"), 0);
///
pub fn atoi(s: &str) -> i32 {
    let mut digits = String::new();
    if let Some(first) = s.split_whitespace().next() {
        for (i, c) in first.chars().enumerate() {
            if i == 0 && (c == '+' || c == '-') {
                digits.push(c);
            } else if c.is_ascii_digit() {
                digits.push(c);
            } else {
                break
            }
        }
    }
        
    match digits.parse::<i32>() {
        Ok(num) => num,
        Err(err) => {
            // Unfortunately, ParseIntError is a struct, not an enum, so we can't match here
            if err.to_string() == "number too small to fit in target type" {
                i32::MIN
            } else if err.to_string() == "number too large to fit in target type" {
                i32::MAX
            } else {
                0
            }
        }
    }
}    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atoi_returns_i32_min_for_out_of_range_negative_int() {
        let s = "-91283472332";
        let expected = i32::MIN;
        let got = atoi(s);
        assert_eq!(got, expected);
    }

    #[test]
    fn atoi_returns_i32_max_for_out_of_range_positive_int() {
        let s = "+91283472332";
        let expected = i32::MAX;
        let got = atoi(s);
        assert_eq!(got, expected);
    }
}
