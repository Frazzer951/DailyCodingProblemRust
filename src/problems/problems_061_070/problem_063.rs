/* EASY
Given a 2D matrix of characters and a target word, write a function that returns
whether the word can be found in the matrix by going left-to-right, or
up-to-down.

For example, given the following matrix:

[['F', 'A', 'C', 'I'],
 ['O', 'B', 'Q', 'P'],
 ['A', 'N', 'O', 'B'],
 ['M', 'A', 'S', 'S']]


and the target word 'FOAM', you should return true, since it's the leftmost
column. Similarly, given the target word 'MASS', you should return true, since
it's the last row.
*/

fn check_word_right(arr: &[Vec<char>], r: usize, c: usize, word: &str) -> bool {
    let word_len = word.len();
    let row_len = arr[0].len();
    if word_len != row_len - c {
        return false;
    }
    for i in 0..word_len {
        let cur_char = word.chars().nth(i).unwrap();
        if cur_char != arr[r][i + c] {
            return false;
        }
    }
    true
}

fn check_word_down(arr: &[Vec<char>], r: usize, c: usize, word: &str) -> bool {
    let word_len = word.len();
    let col_len = arr.len();
    if word_len != col_len - r {
        return false;
    }
    for i in 0..word_len {
        let cur_char = word.chars().nth(i).unwrap();
        if cur_char != arr[i + r][c] {
            return false;
        }
    }
    true
}

fn problem_063(arr: Vec<Vec<char>>, word: &str) -> bool {
    for r in 0..arr.len() {
        for c in 0..arr[r].len() {
            if check_word_right(&arr, r, c, word) {
                return true;
            }
            if check_word_down(&arr, r, c, word) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_063() {
        assert!(problem_063(vec![vec!['F', 'A', 'C', 'I'],
                                 vec!['O', 'B', 'Q', 'P'],
                                 vec!['A', 'N', 'O', 'B'],
                                 vec!['M', 'A', 'S', 'S']],
                            "FOAM"));
        assert!(problem_063(vec![vec!['F', 'A', 'C', 'I'],
                                 vec!['O', 'B', 'Q', 'P'],
                                 vec!['A', 'N', 'O', 'B'],
                                 vec!['M', 'A', 'S', 'S']],
                            "MASS"));
        assert!(!problem_063(vec![vec!['F', 'A', 'C', 'I'],
                                  vec!['O', 'B', 'Q', 'P'],
                                  vec!['A', 'N', 'O', 'B'],
                                  vec!['M', 'A', 'S', 'S']],
                             "CARS"));
    }
}
