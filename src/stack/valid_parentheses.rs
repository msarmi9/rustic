//! Valid Parentheses

use std::collections::HashMap;
use std::collections::HashSet;


/// Checks if a string has valid pairs of open and close parentheses.
///
/// # Examples
///
///     # use rustic::stack::valid_parentheses::has_valid_parentheses;
///     assert!(!has_valid_parentheses("[("));
///     assert!(!has_valid_parentheses("]}("));
///     assert!(!has_valid_parentheses("[[{]]"));
///     assert!(has_valid_parentheses("([{}])"));
///
pub fn has_valid_parentheses(s: &str) -> bool {
    let pmap: HashMap<char, char> = [(')', '('), ('}', '{'), (']', '[')].iter().cloned().collect();
    let pleft: HashSet<char> = pmap.values().cloned().collect();
    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        if pleft.contains(&c) {
            stack.push(c);
        }

        if let Some(&left) = pmap.get(&c) {
            match stack.pop() {
                Some(last) => if last != left { return false },
                None => return false,
            }
        }
    }

    stack.len() == 0
}
