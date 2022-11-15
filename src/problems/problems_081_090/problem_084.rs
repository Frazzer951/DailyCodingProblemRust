/* MEDIUM
Given a matrix of 1s and 0s, return the number of "islands" in the matrix. A 1
represents land and 0 represents water, so an island is a group of 1s that are
neighboring whose perimeter is surrounded by water.

For example, this matrix has 4 islands.

1 0 0 0 0
0 0 1 1 0
0 1 1 0 0
0 0 0 0 0
1 1 0 0 1
1 1 0 0 1
*/

use std::collections::VecDeque;

fn num_islands(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = if m == 0 { 0 } else { grid[0].len() };
    let offsets = vec![0, 1, 0, -1, 0];
    let mut islands = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                islands += 1;
                grid[i][j] = 0;
                let mut todo = VecDeque::from([(i, j)]);
                while !todo.is_empty() {
                    let p = todo.pop_front().unwrap();
                    for k in 0..4 {
                        let r = p.0 as i32 + offsets[k];
                        let c = p.1 as i32 + offsets[k + 1];
                        if r >= 0
                            && r < m as i32
                            && c >= 0
                            && c < n as i32
                            && grid[r as usize][c as usize] == 1
                        {
                            grid[r as usize][c as usize] = 0;
                            todo.push_back((r as usize, c as usize));
                        }
                    }
                }
            }
        }
    }

    islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        assert_eq!(
            num_islands(vec![
                vec![1, 0, 0, 0, 0],
                vec![0, 0, 1, 1, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 1],
                vec![1, 1, 0, 0, 1]
            ]),
            4
        );
    }
}
