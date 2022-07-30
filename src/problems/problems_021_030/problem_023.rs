/* EASY
You are given an M by N matrix consisting of booleans that represents a board.
Each True boolean represents a wall. Each False boolean represents a tile you
can walk on.

Given this matrix, a start coordinate, and an end coordinate, return the minimum
number of steps required to reach the end coordinate from the start. If there is
no possible path, then return null. You can move up, left, down, and right. You
cannot move through walls. You cannot wrap around the edges of the board.

For example, given the following board:

[[f, f, f, f],
[t, t, f, t],
[f, f, f, f],
[f, f, f, f]]


and start = (3, 0) (bottom left) and end = (0, 0) (top left), the minimum number
of steps required to reach the end is 7, since we would need to go through (1,
2) because there is a wall everywhere else on the second row.
*/

use std::collections::hash_set::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i64, i64);

fn get_walkable_neighbors(board: &[Vec<bool>], point: &Point) -> Vec<Point> {
    let row = point.0;
    let col = point.1;
    let mut neighbors = Vec::new();
    if row > 0 && !board[row as usize - 1][col as usize] {
        neighbors.push(Point(row - 1, col));
    }
    if col > 0 && !board[row as usize][col as usize - 1] {
        neighbors.push(Point(row, col - 1));
    }
    if row < board.len() as i64 - 1 && !board[row as usize + 1][col as usize] {
        neighbors.push(Point(row + 1, col));
    }
    if col < board[0].len() as i64 - 1 && !board[row as usize][col as usize + 1] {
        neighbors.push(Point(row, col + 1));
    }
    neighbors
}

fn shortest_path(board: Vec<Vec<bool>>, start: Point, end: Point) -> Option<i64> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(start, 0)]);

    while !queue.is_empty() {
        let (coords, count) = queue.pop_front().unwrap();
        if coords == end {
            return Some(count);
        }
        seen.insert(coords.clone());
        let neighbors = get_walkable_neighbors(&board, &coords);
        for neighbor in neighbors {
            if !seen.contains(&neighbor) {
                queue.push_back((neighbor, count + 1));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_023_1() {
        assert_eq!(
            shortest_path(
                vec![
                    vec![false, false, false, false],
                    vec![true, true, false, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                Point(3, 0),
                Point(0, 0),
            ),
            Some(7)
        );
    }

    #[test]
    fn test_problem_023_2() {
        assert_eq!(
            shortest_path(
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                Point(3, 0),
                Point(0, 0),
            ),
            None
        );
    }
}
