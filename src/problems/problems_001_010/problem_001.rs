/* EASY
Given a list of numbers and a number k, return whether any two numbers from the
list add up to k.

For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

Bonus: Can you do this in one pass?
*/

fn problem_001(arr: Vec<i64>, k: i64) -> bool {
    for i in 0..arr.len() {
        let x = arr[i];
        let j = (k - x).abs();
        if arr.contains(&j) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_001() {
        assert_eq!(problem_001(vec! {10,15,3,7}, 17), true);
        assert_eq!(problem_001(vec! {10,15,3,7}, 16), false);
    }
}
