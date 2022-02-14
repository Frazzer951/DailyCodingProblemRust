/* HARD
Given a list of integers, write a function that returns the largest sum of
non-adjacent numbers. Numbers can be 0 or negative.

For example, [2, 4, 6, 2, 5] should return 13, since we pick 2, 6, and 5. [5, 1,
1, 5] should return 10, since we pick 5 and 5.

Follow-up: Can you do this in O(N) time and constant space?
*/

use std::cmp::max;

fn problem_009(v: Vec<i64>) -> i64 {
    if v.len() <= 2 {
        return max(0, v[0]);
    }

    let mut max_excluding_last = max(0, v[0]);
    let mut max_including_last = max(max_excluding_last, v[1]);

    for i in v.iter().skip(2) {
        let prev_max_including_last = max_including_last;
        max_including_last = max(max_including_last, max_excluding_last + i);
        max_excluding_last = prev_max_including_last;
    }

    max(max_including_last, max_excluding_last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_009() {
        assert_eq!(problem_009(vec! {2, 4, 6, 2, 5}), 13);
        assert_eq!(problem_009(vec! {5, 1, 1, 5}), 10);
        assert_eq!(problem_009(vec! {5, 1, 1, 5, 1, 3}), 13);
        assert_eq!(problem_009(vec! {1, 7, 3, 4, 1, 1, 10}), 21);
    }
}
