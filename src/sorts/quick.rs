//! Quick Sort

/// In-place quick sort.
///
/// # Complexity
///
/// Best: O(n*log(n))
/// Average: O(n*log(n))
/// Worst: O(n^2)
///
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return };
    let idx = partition(arr);
    quicksort(&mut arr[..idx]);
    quicksort(&mut arr[idx..]);
}


/// In-place partition using the final index as the pviot. Returns the pivot's new index.
/// After partitioning, elements to the left are <= to the pivot and elements to the right are >=.
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    // [5, 4, 3, 2, 1]
    let pivot = arr.len() - 1;
    let mut right = pivot - 1;
    let mut left = 0;
    loop {
        while arr[left] < arr[pivot] {
            left += 1;
        }

        while arr[right] > arr[pivot] && right > 0 {
            right -= 1;
        }

        if left < right {
            arr.swap(left, right);
            left += 1;
        } else {
            break
        }
    }

    arr.swap(left, pivot);
    left
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_on_positive_ints() {
        let mut arr = [0, 5, 2, 1, 6, 3];
        let got = partition(&mut arr);
        let expected = 3;
        assert_eq!(got, expected);
    }

    #[test]
    fn partition_on_descending_ints() {
        let mut arr = [5, 4, 3, 2, 1];
        let got = partition(&mut arr);
        let expected = 0;
        assert_eq!(got, expected);
    }

    #[test]
    fn quicksort_on_positive_ints() {
        let mut arr = [0, 5, 2, 1, 6, 3];
        let expected = [0, 1, 2, 3, 5, 6];
        quicksort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn quicksort_on_mixed_ints() {
        let mut arr = [-3, 2, 4, -1, 0];
        let expected = [-3, -1, 0, 2, 4];
        quicksort(&mut arr);
        assert_eq!(arr, expected);
    }

    #[test]
    fn quicksort_on_descending_ints() {
        let mut arr = [5, 4, 3, 2, 1];
        let expected = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, expected);
    }
}
