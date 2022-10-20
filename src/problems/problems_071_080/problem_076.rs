/* MEDIUM
You are given an N by M 2D matrix of lowercase letters. Determine the minimum
number of columns that can be removed to ensure that each row is ordered from
top to bottom lexicographically. That is, the letter at each column is
lexicographically later as you go down each row. It does not matter whether each
row itself is ordered lexicographically.

For example, given the following table:

cba
daf
ghi


This is not ordered because of the a in the center. We can remove the second
column to make it ordered:

ca
df
gi


So your function should return 1, since we only needed to remove 1 column.

As another example, given the following table:

abcdef


Your function should return 0, since the rows are already ordered (there's only
one row).

As another example, given the following table:

zyx
wvu
tsr


Your function should return 3, since we would need to remove all the columns to
order it.
*/

fn cols_to_remove(mat: Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    if mat.len() <= 1 {
        return 0;
    }

    let rows = mat.len();
    let cols = mat[0].len();

    for j in 0..cols {
        for i in 1..rows {
            if mat[i][j] < mat[i - 1][j] {
                count += 1;
                break;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cols_to_remove_1() {
        let result = cols_to_remove(vec![vec!['c', 'b', 'a'], vec!['d', 'a', 'f'], vec!['g', 'h', 'i']]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_cols_to_remove_2() {
        let result = cols_to_remove(vec![vec!['a', 'b', 'c', 'd', 'e', 'f']]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_cols_to_remove_3() {
        let result = cols_to_remove(vec![vec!['z', 'y', 'x'], vec!['w', 'v', 'u'], vec!['t', 's', 'r']]);
        assert_eq!(result, 3);
    }
}
