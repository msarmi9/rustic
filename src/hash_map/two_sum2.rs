// Two Sum Sorted
//
// Given an array `nums` of ints sorted in ascending order and an int `target`, return the indices
// of the two numbers whose sum is `target`. You may assume there is only one such pair and you may
// not use the same number twice.

use std::collections::HashMap;


pub fn two_sum_ascending(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();
    for (i, a) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(a) {
            return Some((j, i))
        } else if *a > target {
            break
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
        let (got, expected) = (two_sum_ascending(nums, target), Some((0, 1)));
        assert_eq!(got, expected);
    }

    #[test]
    fn negative_nums() {
        let (nums, target) = (vec![-3, -2, -1, 0], -1);
        let (got, expected) = (two_sum_ascending(nums, target), Some((2, 3)));
        assert_eq!(got, expected);
    }

    #[test]
    fn postive_and_negative_nums() {
        let (nums, target) = (vec![-5, -2, 0, 3, 7], 1);
        let (got, expected) = (two_sum_ascending(nums, target), Some((1, 3)));
        assert_eq!(got, expected);
    }
}



