//! Insertion Sort

/// In-place insertion sort. Each iteration inserts the current element into its correct position
/// relative to the elements to its *left*.
///
/// # Complexity
///
/// Best: O(n)
/// Average: O(n^2 / 2)
/// Worst: O(n^2)
///
/// # Examples
///
///     # use rustic::sorts::insertion::insertion_sort;
///     assert_eq!(insertion_sort(&mut [4, 2, 7, 1, 3]), [1, 2, 3, 4, 7]);
///
pub fn insertion_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for end in 1 .. arr.len() {
        let mut i = end;
        while i > 0 && arr[i - 1] > arr[i] {
            arr.swap(i - 1, i);
            i -= 1;
        }
    }
    arr
}
