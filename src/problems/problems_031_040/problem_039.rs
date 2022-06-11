/* MEDIUM
Conway's Game of Life takes place on an infinite two-dimensional board of square
cells. Each cell is either dead or alive, and at each tick, the following rules
apply:

 * Any live cell with less than two live neighbours dies.
 * Any live cell with two or three live neighbours remains living.
 * Any live cell with more than three live neighbours dies.
 * Any dead cell with exactly three live neighbours becomes a live cell.

A cell neighbours another cell if it is horizontally, vertically, or diagonally
adjacent.

Implement Conway's Game of Life. It should be able to be initialized with a
starting list of live cell coordinates and the number of steps it should run
for. Once initialized, it should print out the board state at each step. Since
it's an infinite board, print out only the relevant coordinates, i.e. from the
top-leftmost live cell to bottom-rightmost live cell.

You can represent a live cell with an asterisk (*) and a dead cell with a dot (.
).
*/

fn count_neighbors(board: &[Vec<bool>], i: i64, j: i64) -> usize {
    let mut count = 0;
    for x in i - 1..i + 2 {
        for y in j - 1..j + 2 {
            if x == i && y == j {
                continue;
            }
            if x < board.len() as i64 && y < board[0].len() as i64 && x >= 0 && y >= 0 && board[x as usize][y as usize] {
                count += 1;
            }
        }
    }
    count
}

fn problem_039(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = board.len();
    let height = board[0].len();
    let mut new_board = vec![vec![false; height]; width];

    for i in 0..width {
        for j in 0..height {
            let neighbors = count_neighbors(&board, i as i64, j as i64);
            if board[i][j] {
                if !(2..=3).contains(&neighbors) {
                    new_board[i][j] = false;
                } else {
                    new_board[i][j] = true;
                }
            } else if neighbors == 3 {
                new_board[i][j] = true;
            }
        }
    }

    new_board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_039() {
        assert_eq!(
            problem_039(vec![
                vec![false, true, false],
                vec![false, true, false],
                vec![false, true, false],
            ]),
            vec![
                vec![false, false, false],
                vec![true, true, true],
                vec![false, false, false],
            ]
        );
    }
}
