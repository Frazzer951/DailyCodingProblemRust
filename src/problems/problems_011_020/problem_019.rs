/* MEDIUM
A builder is looking to build a row of N houses that can be of K different
colors. He has a goal of minimizing cost while ensuring that no two neighboring
houses are of the same color.

Given an N by K matrix where the nth row and kth column represents the cost to
build the nthhouse with kth color, return the minimum cost which achieves this
goal.
*/

fn problem_019(matrix: Vec<Vec<i64>>) -> i64 {
    let mut lowest_cost = 0;
    let mut lowest_cost_index = matrix[0].len();
    let mut second_lowest_cost = 0;

    for row in matrix.iter() {
        let mut new_lowest_cost = i64::MAX;
        let mut new_lowest_cost_index = matrix[0].len();
        let mut new_second_lowest_cost = i64::MAX;
        for (c, val) in row.iter().enumerate() {
            let prev_lowest_cost = if c == lowest_cost_index {
                second_lowest_cost
            } else {
                lowest_cost
            };
            let cost = prev_lowest_cost + *val;
            if cost < new_lowest_cost {
                new_second_lowest_cost = new_lowest_cost;
                new_lowest_cost = cost;
                new_lowest_cost_index = c;
            } else if cost < new_second_lowest_cost {
                new_second_lowest_cost = cost;
            }
        }
        lowest_cost = new_lowest_cost;
        lowest_cost_index = new_lowest_cost_index;
        second_lowest_cost = new_second_lowest_cost;
    }
    lowest_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_019() {
        assert_eq!(problem_019(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), 13);
        assert_eq!(problem_019(vec![vec![10, 15, 20], vec![14, 47, 6], vec![2, 7, 10]]), 18);
        assert_eq!(problem_019(vec![vec![10, 15, 20], vec![6, 47, 14], vec![2, 7, 10]]), 26);
    }
}
