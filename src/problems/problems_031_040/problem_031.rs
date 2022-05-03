/* EASY
The edit distance between two strings refers to the minimum number of character
insertions, deletions, and substitutions required to change one string to the
other. For example, the edit distance between “kitten” and “sitting” is three:
substitute the “k” for “s”, substitute the “e” for “i”, and append a “g”.

Given two strings, compute the edit distance between them.
*/

use std::cmp::min;

fn problem_031(s1: &str, s2: &str) -> i64 {
    let x = s1.len() + 1;
    let y = s2.len() + 1;
    let mut a = vec![vec![-1; x]; y];

    for i in 0..x {
        a[0][i] = i as i64;
    }
    for (j, arr) in a.iter_mut().enumerate().take(y) {
        arr[0] = j as i64;
    }

    for i in 1..y {
        for j in 1..x {
            if s1.chars().nth(j - 1).unwrap() == s2.chars().nth(i - 1).unwrap() {
                a[i][j] = a[i - 1][j - 1];
            } else {
                a[i][j] = min(min(a[i - 1][j] + 1, a[i][j - 1] + 1), a[i - 1][j - 1] + 1);
            }
        }
    }

    a[y - 1][x - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_031() {
        assert_eq!(problem_031("kitten", "sitting"), 3);
    }
}
