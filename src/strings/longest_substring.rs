//! Longest Substring Without Repeating Characters

use std::collections::HashMap;


/// Returns the length of the longest substring with no repeating chracters.
///
/// # Examples
///
///     # use rustic::strings::longest_substring::longest_substring;
///     assert_eq!(longest_substring("bbb"), 1);
///     assert_eq!(longest_substring("abcbb"), 3);
///     assert_eq!(longest_substring("dvdf"), 3);
///     assert_eq!(longest_substring("avdydfvx"), 5);
///
pub fn longest_substring(s: &str) -> usize {
    let mut longest = 0;
    let mut i = 0;
    let mut seen = HashMap::new();

    for (j, c) in s.chars().enumerate() {
        if let Some(k) = seen.get(&c) {
            i = i.max(k + 1);
        }

        seen.insert(c, j);
        longest = longest.max(j - i + 1);
    }
    
    longest
}
