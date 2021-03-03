//! Selection Sort

/// In-place selection sort. Each iteration selects the smallest element to the right of the
/// current element and places it in its final sorted position at the front of the array.
///
// # Complexity
//
// - Best: O(n^2 / 2)
// - Average: O(n^2 / 2)
// - Worst: O(n^2 / 2)
//
/// # Examples
///
///     # use rustic::sorts::selection::selection_sort;
///     assert_eq!(selection_sort(&mut [3, -1, 2, 0, 4]), [-1, 0, 2, 3, 4]);
///     assert_eq!(selection_sort(&mut [4, 3, 2, 1]), [1, 2, 3, 4]);
///
pub fn selection_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for start in 0 .. arr.len() {
        let mut smallest = start;
        for right in start + 1 .. arr.len() {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(start, smallest);
    }
    arr
}
