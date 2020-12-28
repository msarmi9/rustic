//! Two Sum 
//!
//! Given an array `nums` of ints and an int `target`, return the indices of the two numbers whose
//! sum is `target`. You may assume there is only one such pair.

use std::collections::HashMap;


pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Maps the complement of each number we've seen to the number's index
    // If a number is already a key, we know we've seen its complement 
    let mut seen = HashMap::new();
    for (i, a) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(a) {
            return Some((j, i))
        } else {
            seen.insert(target - a, i);
        }
    }
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_nums() {
        let (nums, target) = (vec![2, 7, 11, 15], 9);
        let (got, expected) = (two_sum(&nums, target), Some((0, 1)));
        assert_eq!(got, expected);
    }

    #[test]
    fn negative_nums() {
        let (nums, target) = (vec![-3, -5, -2], -5);
        let (got, expected) = (two_sum(&nums, target), Some((0, 2)));
        assert_eq!(got, expected);
    }

    #[test]
    fn positive_and_negative_nums() {
        let (nums, target) = (vec![-3, 3, 9, -1, 0], 2);
        let (got, expected) = (two_sum(&nums, target), Some((1, 3)));
        assert_eq!(got, expected);
    }
}
