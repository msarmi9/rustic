//! Coffee Scheduler
//!
//! Meet Lara & Lars, two good friends and two very busy engineers. Lara & Lars like to grab coffee
//! once a week to catch up, but it's a total headache to find a time during which they're both
//! free, so they decide to automate the pain away.
//!
//! Their daily free time can be represented as a sorted array of disjoint time intervals, with
//! each interval indicating a free block's start and end time. For example, Monday's free blocks
//! look like:
//!
//! Lara: [(8, 9), (12, 13), (15, 17), (18, 20)]
//! Lars: [(7, 8), (10, 13), (14, 16), (17, 18), (19, 22)]
//!
//! The times where they're both free to grab a coffee look like:
//!
//! Coffee: [(12, 13), (15, 16), (19, 20)]
//!
//! Write a function that takes their scheduled free time as input and returns a single array
//! containing all blocks during which they're both free.


type Block = (u8, u8);

#[allow(non_snake_case)]
pub fn intersect(A: &[Block], B: &[Block]) -> Vec<Block> {
    let mut i = 0;
    let mut j = 0;
    let mut start: u8;
    let mut intersection = Vec::new();

    while i < A.len() && j < B.len() {
        let a = A[i];
        let b = B[j];

        if a.1 < b.1 {
            if a.1 > b.0 {
                start = a.0.max(b.0);
                intersection.push((start, a.1));
            }
            i += 1;
        } else {
            if a.0 < b.1 {
                start = a.0.max(b.0);
                intersection.push((start, b.1));
            }
            j += 1;
        }
    }

    intersection
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Case where each block intersects at most one other block.
    fn blocks_intersect_at_most_one_block() {
        let lara = [(8, 9), (12, 13), (15, 17), (18, 20)];
        let lars = [(7, 8), (10, 13), (14, 16), (17, 18), (19, 20)];
        let got = intersect(&lara, &lars);
        let expected = vec![(12, 13), (15, 16), (19, 20)];
        assert_eq!(got, expected);
    }

    #[test]
    /// Case where some blocks intersect more than one other block.
    fn blocks_intersect_more_than_one_block() {
        let lara = [(9, 11), (12, 15), (16, 19)];
        let lars = [(8, 9), (10, 13), (14, 17)];
        let got = intersect(&lara, &lars);
        let expected = vec![(10, 11), (12, 13), (14, 15), (16, 17)];
        assert_eq!(got, expected);
    }

    #[test]
    /// Case where no block intersects another block.
    fn no_block_intersects_another_block() {
        let lara = [(9, 11), (12, 15), (16, 19)];
        let lars = [(8, 9), (11, 12), (15, 16)];
        let got = intersect(&lara, &lars);
        let expected = vec![];
        assert_eq!(got, expected);
    }

    #[test]
    /// Case where one block intersects all other blocks.
    fn one_block_intersects_all_other_blocks() {
        let lara = [(0, 24)];
        let lars = [(5, 8), (12, 14), (16, 19)];
        let got = intersect(&lara, &lars);
        let expected = &lars;
        assert_eq!(got, expected);
    }
}
