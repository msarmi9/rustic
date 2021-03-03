//! Bubble Sort

/// In-place bubble sort. Each iteration "bubbles up" the highest unsorted value and places it in
/// its final sorted position at the end of the array.
///
/// # Examples
///
///     # use rustic::sorts::bubble::bubble_sort;
///     assert_eq!(bubble_sort(&mut [2, 1, 4, 3, 5]), [1, 2, 3, 4, 5]);
///     assert_eq!(bubble_sort(&mut [-1, 11, 7, 3, 0]), [-1, 0, 3, 7, 11]);
///     assert_eq!(bubble_sort(&mut ['c', 'a', 'k', 'e']), ['a', 'c', 'e', 'k']);
///
pub fn bubble_sort<T: Ord>(arr: &mut [T]) -> &[T] {
    for end in (0 .. arr.len()).rev() {
        for i in 0 .. end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
    arr
}


// In-place bubble sort with early return.
//
// # Complexity
///
/// - Best: O(n)
/// - Average: O(n^2)
/// - Worst: O(n^2)
///
/// # Examples
///
///     # use rustic::sorts::bubble::bubble_sort2;
///     assert_eq!(bubble_sort2(&mut [2, 1, 4, 3, 5]), [1, 2, 3, 4, 5]);
///
pub fn bubble_sort2<T: Ord>(arr: &mut [T]) -> &[T] {
    let mut end = arr.len() - 1;
    let mut sorted = false;

    while !sorted{
        sorted = true;
        for i in 0 .. end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
            end -= 1;
        }
    }
    arr
}
