/* MEDIUM
There is an N by M matrix of zeroes. Given N and M, write a function to count
the number of ways of starting at the top-left corner and getting to the
bottom-right corner. You can only move right or down.

For example, given a 2 by 2 matrix, you should return 2, since there are two
ways to get to the bottom-right:

 * Right, then down
 * Down, then right

Given a 5 by 5 matrix, there are 70 ways to get to the bottom-right.
*/

fn problem_062(n: usize, m: usize) -> i64 {
    let mut arr = vec![vec![0; m]; n];
    for i in arr.iter_mut().take(n) {
        i[0] = 1;
    }
    for i in 0..m {
        arr[0][i] = 1;
    }

    for i in 1..n {
        for j in 1..m {
            arr[i][j] = arr[i - 1][j] + arr[i][j - 1];
        }
    }

    arr[n - 1][m - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_062() {
        assert_eq!(problem_062(5, 5), 70);
    }
}
