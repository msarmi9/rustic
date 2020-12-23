// Three Sum
//
// Given an array `nums` of ints, are there elements a, b, c such that a + b + c = 0?
// Find all such unique triples in the array `nums`.


/// Searches `nums` for all pairs of entries whose sum equals `target`.
/// Assumes `nums` is non-empty and sorted in ascending order.
pub fn two_sum2(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut pairs = vec![];
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r && nums[l] <= target {
        if l > 0 && nums[l] == nums[l-1] {
            l += 1;
        } else if nums[l] + nums[r] < target {
            l += 1;
        } else if nums[l] + nums[r] > target {
            r += 1;
        } else {
            pairs.push((nums[l], nums[r]));
            l += 1;
            r -= 1;
        }
    }
    pairs
}


/// Searches `nums` for all unique triples whose sum is zero.
pub fn three_sum(mut nums: Vec<i32>) -> Vec<(i32, i32, i32)> {
    nums.sort();
    let mut triples = vec![];
    for (i, &a) in nums.iter().enumerate() {
        if a > 0 {
            break
        } else if i == 0 || a != nums[i-1] {
            if nums[i+1..].len() == 0 {
                break
            } else {
                let doubles = two_sum2(&nums[i+1..], -a);
                if !doubles.is_empty() {
                    triples.extend(doubles.iter().map(|&(b, c)| (a, b, c)));
                }
            }
        }
    }
    triples
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_on_mixed_nums() {
        let nums = vec![-4, -1, -1, 0, 1, 2];
        let expected = vec![(-1, -1, 2), (-1, 0, 1)];
        let got = three_sum(nums);
        assert_eq!(got, expected);
    }

    #[test]
    fn three_sum_on_all_zero_vec() {
        let nums = vec![0, 0, 0, 0, 0];
        let expected = vec![(0, 0, 0)];
        let got = three_sum(nums);
        assert_eq!(got, expected);
    }

    #[test]
    fn three_sum_returns_empty_vec_for_singleton_vec() {
        let nums = vec![0];
        let expected = vec![];
        let got = three_sum(nums);
        assert_eq!(got, expected);
    }

    #[test]
    fn three_sum_returns_empty_vec_for_empty_vec() {
        let nums = vec![];
        let expected = vec![];
        let got = three_sum(nums);
        assert_eq!(got, expected);
    }

    #[test]
    fn three_sum_returns_empty_vec_for_positive_nums() {
        let nums = vec![1, 2, 3];
        let expected = vec![];
        let got = three_sum(nums);
        assert_eq!(got, expected);
    }
}
