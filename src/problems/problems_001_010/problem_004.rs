/* HARD
Given an array of integers, find the first missing positive integer in linear
time and constant space. In other words, find the lowest positive integer that
does not exist in the array. The array can contain duplicates and negative
numbers as well.

For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should
give 3.

You can modify the input array in-place.
*/

fn problem_004(mut arr: Vec<i64>) -> i64 {
    arr.sort();
    let mut missing = 1;
    for i in arr {
        if i > 0 && i == missing {
            missing += 1;
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_004() {
        assert_eq!(problem_004(vec! {3,4,-1,1}), 2);
        assert_eq!(problem_004(vec! {1,2,0}), 3);
    }
}
