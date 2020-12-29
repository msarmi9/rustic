//! Subarrays Divisible By K

/// Returns the number of contiguous subarrays whose sum is divisible by a positive integer `k`.
pub fn subarrays_div_by_k(nums: &[i32], k: usize) -> usize {
    // Count, for each index i, the sums mod k of all i+1 contiguous subarrays ending at i.
    let mut count = 0;
    let mut residues = vec![0; k];
    for num in nums {
        let r = num.rem_euclid(k as i32) as usize;
        residues.rotate_right(r);
        residues[r] += 1;
        count += residues[0];
    }
    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarrays_div_by_3() {
        let (nums, k) = ([5, 3, 7, 2, 8], 3);
        let expected = [vec![3], vec![5, 3, 7], vec![3, 7, 2], vec![7, 2]].len();
        let got = subarrays_div_by_k(&nums, k);
        assert_eq!(got, expected);
    }

    #[test]
    fn subarrays_div_by_5() {
        let (nums, k) = ([4, 11, -2, 5, 9], 5);
        let expected = [vec![4, 11], vec![5]].len();
        let got = subarrays_div_by_k(&nums, k);
        assert_eq!(got, expected);
    }
}
