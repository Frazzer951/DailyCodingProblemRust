/* HARD
Given an array of integers and a number k, where 1 <= k <= length of the array,
compute the maximum values of each subarray of length k.

For example, given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7,
8, 8], since:

 * 10 = max(10, 5, 2)
 * 7 = max(5, 2, 7)
 * 8 = max(2, 7, 8)
 * 8 = max(7, 8, 7)

Do this in O(n) time and O(k) space. You can modify the input array in-place and
you do not need to store the results. You can simply print them out as you
compute them.
*/

fn problem_018(arr: Vec<i64>, k: usize) -> Vec<i64> {
    let mut max_arr = vec![];

    for i in 0..=arr.len() - k {
        let mut sub_arr = vec![];
        for j in arr.iter().skip(i).take(k) {
            sub_arr.push(*j);
        }
        max_arr.push(*sub_arr.iter().max().unwrap());
    }

    max_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_018() {
        // given array = [10, 5, 2, 7, 8, 7] and k = 3, we should get: [10, 7, 8, 8]
        assert_eq!(problem_018(vec![10, 5, 2, 7, 8, 7], 3), vec![10, 7, 8, 8]);
    }
}
