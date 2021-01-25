//! Subarray Sum Equals K

use std::collections::HashMap;


/// Returns the number of contiguous subarrays whose sum equals an integer `k`.
///
/// # Examples
///
///     # use rustic::dynp::subarrays_sum_equals_k::subarrays_summing_to_k;
///     assert_eq!(subarrays_summing_to_k(&[1, 1, 1], 2), 2);
///     assert_eq!(subarrays_summing_to_k(&[1, 2, 3], 3), 2);
///     assert_eq!(subarrays_summing_to_k(&[1, -3], 3), 0);
///
pub fn subarrays_summing_to_k(nums: &[i32], k: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut seen = HashMap::with_capacity(nums.len());
    seen.insert(0, 1);

    for &num in nums {
        sum += num;
        count += seen.get(&(sum - k)).unwrap_or(&0);
        *seen.entry(sum).or_insert(0) += 1;
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarrays_summing_to_neg_3() {
        let (nums, k) = ([-3, 0, 2, -2, -1], -1);
        let expected = [vec![-3, 0, 2], vec![0, 2, -2, -1], vec![2, -2, -1], vec![-1]].len() as i32;
        let got = subarrays_summing_to_k(&nums, k);
        assert_eq!(&got, &expected);
    }

    #[test]
    fn subarrays_summing_to_5() {
        let (nums, k) = ([2, 1, 3, -1, 3], 5);
        let expected = [vec![2, 1, 3, -1], vec![3, -1, 3]].len() as i32;
        let got = subarrays_summing_to_k(&nums, k);
        assert_eq!(&got, &expected);
    }
}

