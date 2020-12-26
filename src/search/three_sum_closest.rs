//! Three Sum Closest
//!
//! Given an array `nums` of ints and an int `target`, find the three numbers whose sum is
//! closest to `target` and return their sum.

pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums; nums.sort();
    let mut closest = nums[..3].iter().sum::<i32>();
    for i in 0..nums.len()-2 {
        if i > 0 && nums[i] == nums[i-1] {
            continue
        }
        
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let sum = nums[i] + nums[l] + nums[r];
            if (target - sum).abs() < (target - closest).abs() {
                closest = sum;
            }

            if sum < target {
                l += 1;
            } else if sum > target {
                r -= 1;
            } else {
                return sum
            }
        } 
    }
    closest
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_closest_on_vec_without_target() {
        let (nums, target) = (vec![1, 2, 5, 10, 11], 12);
        let expected = 13;
        let got = three_sum_closest(nums, target);
        assert_eq!(got, expected);
    }

    #[test]
    fn three_sum_closest_on_vec_with_target() {
        let (nums, target) = (vec![1, 2, 3], 6);
        let expected = target;
        let got = three_sum_closest(nums, target);
        assert_eq!(got, expected);
    }
}
