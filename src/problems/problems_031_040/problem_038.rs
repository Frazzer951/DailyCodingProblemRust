/* HARD
You have an N by N board. Write a function that, given N, returns the number of
possible arrangements of the board where N queens can be placed on the board
without threatening each other, i.e. no two queens share the same row, column,
or diagonal.
*/

fn is_valid(board: Vec<usize>) -> bool {
    let current_queen_row = (board.len() - 1) as i64;
    let current_queen_col = board[board.len() - 1] as i64;

    print!("Board:");
    print!("{:#?}", board);

    for (row, val) in board.iter().enumerate().take(board.len() - 1) {
        let diff = (current_queen_col - *val as i64).abs();
        if diff == 0 || diff == (current_queen_row - row as i64) {
            return false;
        }
    }

    true
}

fn problem_038_helper(n: usize, mut board: Vec<usize>) -> i64 {
    if n == board.len() {
        return 1;
    }

    let mut count = 0;

    for col in 0..n {
        board.push(col);
        if is_valid(board.clone()) {
            count += problem_038_helper(n, board.clone());
        }
        board.pop();
    }

    count
}

fn problem_038(n: usize) -> i64 {
    problem_038_helper(n, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_038() {
        assert_eq!(problem_038(4), 2);
        assert_eq!(problem_038(8), 92);
    }
}
