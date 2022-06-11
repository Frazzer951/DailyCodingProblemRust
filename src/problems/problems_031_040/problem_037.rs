/* EASY
The power set of a set is the set of all its subsets. Write a function that,
given a set, generates its power set.

For example, given the set {1, 2, 3}, it should return {{}, {1}, {2}, {3}, {1,
2}, {1, 3}, {2, 3}, {1, 2, 3}}.

You may also use a list or array to represent a set.
*/

fn problem_037(set: Vec<i64>) -> Vec<Vec<i64>> {
    let mut power_set = Vec::new();

    let power_set_size = set.len() * set.len() - 1;

    for counter in 0..power_set_size {
        let mut subset = Vec::new();
        for (j, s) in set.iter().enumerate() {
            if (counter & (1 << j)) != 0 {
                subset.push(*s);
            }
        }
        power_set.push(subset);
    }

    power_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_037() {
        assert_eq!(
            problem_037(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }
}
