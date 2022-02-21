/* EASY
Given an array of numbers and a number k, determine if there are three entries
in the array which add up to the specified number k. For example, given [20,
303, 3, 4, 25] and k = 49, returntrue as 20 + 4 + 25 = 49.
*/

fn problem_339(arr: Vec<i64>, k: i64) -> bool {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            for l in j..arr.len() {
                if arr[i] + arr[j] + arr[l] == k {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_339() {
        assert!(problem_339(vec![20, 303, 3, 4, 25], 48));
        assert!(problem_339(vec![20, 303, 3, 4, 25], 49));
        assert!(!problem_339(vec![20, 303, 3, 4, 25], 50));
    }
}
