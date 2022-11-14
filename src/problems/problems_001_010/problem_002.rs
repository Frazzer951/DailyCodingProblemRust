/* HARD
Given an array of integers, return a new array such that each element at index i
of the new array is the product of all the numbers in the original array except
the one at i.

For example, if our input was [1, 2, 3, 4, 5], the expected output would be
[120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be
[2, 3, 6].

Follow-up: what if you can't use division?
*/

fn problem_002(arr: Vec<i64>) -> Vec<i64> {
    let mut new_vec = vec![1; arr.len()];

    for (i, v) in new_vec.iter_mut().enumerate().take(arr.len()) {
        let mut running_prod = 1;
        for (j, a) in arr.iter().enumerate() {
            if i != j {
                running_prod *= a;
            }
        }
        *v = running_prod;
    }

    new_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_002() {
        assert_eq!(
            problem_002(vec! {1, 2, 3, 4, 5}),
            vec! {120, 60, 40, 30, 24}
        );
        assert_eq!(problem_002(vec! {3,2,1}), vec! {2,3,6});
    }
}
