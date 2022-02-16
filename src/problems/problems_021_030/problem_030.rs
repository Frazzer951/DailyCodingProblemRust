/* MEDIUM
You are given an array of non-negative integers that represents a
two-dimensional elevation map where each element is unit-width wall and the
integer is the height. Suppose it will rain and all spots between two walls get
filled up.

Compute how many units of water remain trapped on the map in O(N) time and O(1)
space.

For example, given the input [2, 1, 2], we can hold 1 unit of water in the
middle.

Given the input [3, 0, 1, 3, 0, 5], we can hold 3 units in the first index, 2 in
the second, and 3 in the fourth index (we cannot hold 5 since it would run off
to the left), so we can trap 8 units of water.
*/

use std::cmp::{max, min};

fn problem_030(v: Vec<i64>) -> i64 {
    let mut units_of_water = 0;
    let left_max = v[0];
    let mut right_max = 0;

    for i in 1..v.len() {
        if right_max == v[i] {
            right_max = 0;
        }
        for j in v.iter().skip(i + 1) {
            right_max = max(right_max, *j);
        }
        let temp_water = min(left_max, right_max);

        units_of_water += max(temp_water - v[i], 0);
    }
    units_of_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_030() {
        assert_eq!(problem_030(vec![3, 0, 1, 3, 0, 5]), 8);
    }
}
