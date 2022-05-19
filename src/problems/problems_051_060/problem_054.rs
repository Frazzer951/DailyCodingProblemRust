/* HARD
Sudoku is a puzzle where you're given a partially-filled 9 by 9 grid with
digits. The objective is to fill the grid with the constraint that every row,
column, and box (3 by 3 subgrid) must contain all of the digits from 1 to 9.

Implement an efficient sudoku solver.
*/

fn is_valid_sudoku(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for x in 0..=8 {
        if board[row][x] == num {
            return false;
        }
    }

    for pos in board.iter().take(9) {
        if pos[col] == num {
            return false;
        }
    }

    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

fn solve_sudoku(board: &mut [[u8; 9]; 9], mut row: usize, mut col: usize) -> bool {
    if row == 8 && col == 9 {
        return true;
    }

    if col == 9 {
        row += 1;
        col = 0;
    }

    if board[row][col] > 0 {
        return solve_sudoku(board, row, col + 1);
    }

    for num in 1..=9 {
        if is_valid_sudoku(board, row, col, num) {
            board[row][col] = num;

            if solve_sudoku(board, row, col + 1) {
                return true;
            }
        }
        board[row][col] = 0;
    }

    false
}

fn problem_054(mut board: [[u8; 9]; 9]) -> [[u8; 9]; 9] {
    if solve_sudoku(&mut board, 0, 0) {
        board
    } else {
        println!("{:#?}", board);
        panic!("Sudoku is unsolvable");
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_problem_054() {
        assert_eq!(
            problem_054([
                [0, 0, 0, 0, 9, 3, 0, 0, 0],
                [7, 0, 0, 0, 0, 5, 0, 0, 0],
                [0, 3, 5, 8, 0, 2, 0, 9, 0],
                [4, 5, 0, 2, 0, 0, 6, 0, 0],
                [0, 2, 0, 5, 4, 6, 0, 8, 0],
                [0, 0, 3, 0, 0, 1, 0, 5, 4],
                [0, 4, 0, 1, 0, 8, 9, 3, 0],
                [0, 0, 0, 6, 0, 0, 0, 0, 1],
                [0, 0, 0, 3, 2, 0, 0, 0, 0],
            ]),
            [
                [2, 8, 4, 7, 9, 3, 1, 6, 5],
                [7, 6, 9, 4, 1, 5, 3, 2, 8],
                [1, 3, 5, 8, 6, 2, 4, 9, 7],
                [4, 5, 8, 2, 3, 7, 6, 1, 9],
                [9, 2, 1, 5, 4, 6, 7, 8, 3],
                [6, 7, 3, 9, 8, 1, 2, 5, 4],
                [5, 4, 6, 1, 7, 8, 9, 3, 2],
                [3, 9, 2, 6, 5, 4, 8, 7, 1],
                [8, 1, 7, 3, 2, 9, 5, 4, 6],
            ]
        );
    }
}
