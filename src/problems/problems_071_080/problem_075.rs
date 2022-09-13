/* HARD
Given an array of numbers, find the length of the longest increasing subsequence
in the array. The subsequence does not necessarily have to be contiguous.

For example, given the array [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7,
15], the longest increasing subsequence has length 6: it is 0, 2, 6, 9, 11, 15.
*/

fn longest_increasing_subsequence(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut lis = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] && lis[i] < lis[j] + 1 {
                lis[i] = lis[j] + 1;
            }
        }
    }
    *lis.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_subsequence_1() {
        assert_eq!(
            longest_increasing_subsequence(vec![0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15]),
            6
        );
    }

    #[test]
    fn test_longest_increasing_subsequence_2() {
        assert_eq!(longest_increasing_subsequence(vec![3, 10, 2, 1, 20]), 3);
    }

    #[test]
    fn test_longest_increasing_subsequence_3() {
        assert_eq!(longest_increasing_subsequence(vec![3, 2]), 1);
    }

    #[test]
    fn test_longest_increasing_subsequence_4() {
        assert_eq!(longest_increasing_subsequence(vec![50, 3, 10, 7, 40, 80]), 4);
    }
}
