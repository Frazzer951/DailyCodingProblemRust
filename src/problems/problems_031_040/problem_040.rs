/* HARD
Given an array of integers where every integer occurs three times except for one
integer, which only occurs once, find and return the non-duplicated integer.

For example, given [6, 1, 3, 3, 3, 6, 6], return 1. Given [13, 19, 13, 13],
return 19.

Do this in O(N) time and O(1) space.
*/

fn problem_040(arr: Vec<i32>) -> i32 {
    let mut result_arr = vec![0; 32];
    for num in arr {
        for (i, rarr) in result_arr.iter_mut().enumerate().take(32) {
            let bit = num >> i & 1;
            *rarr = (*rarr + bit) % 3;
        }
    }
    let mut result = 0;
    for (i, rarr) in result_arr.iter_mut().enumerate().take(32) {
        if rarr != &0 {
            result += 1 << i;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_040() {
        assert_eq!(problem_040(vec![6, 1, 3, 3, 3, 6, 6]), 1);
        assert_eq!(problem_040(vec![13, 19, 13, 13]), 19);
    }
}
