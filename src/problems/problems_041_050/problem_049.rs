/* MEDIUM
Given an array of numbers, find the maximum sum of any contiguous subarray of
the array.

For example, given the array [34, -50, 42, 14, -5, 86], the maximum sum would be
137, since we would take elements 42, 14, -5, and 86.

Given the array [-5, -1, -8, -9], the maximum sum would be 0, since we would not
take any elements.

Do this in O(N) time.
*/

use std::cmp::max;

fn problem_049(arr: Vec<i64>) -> i64 {
    let mut max_ending_here = 0;
    let mut max_so_far = 0;
    for x in arr {
        max_ending_here = max(x, max_ending_here + x);
        max_so_far = max(max_so_far, max_ending_here);
    }
    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_049() {
        assert_eq!(problem_049(vec![34, -50, 42, 14, -5, 86]), 137);
        assert_eq!(problem_049(vec![-5, -1, -8, -9]), 0);
    }
}
