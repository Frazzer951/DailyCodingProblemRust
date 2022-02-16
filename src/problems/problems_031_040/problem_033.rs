/* EASY
Compute the running median of a sequence of numbers. That is, given a stream of
numbers, print out the median of the list so far on each new element.

Recall that the median of an even-numbered list is the average of the two middle
numbers.

For example, given the sequence [2, 1, 5, 7, 2, 0, 5], your algorithm should
print out:

2
1.5
2
3.5
2
2
2
*/

fn problem_033(v: Vec<f64>) -> Vec<f64> {
    let mut medians = vec![];
    let mut running = vec![];

    for i in v {
        running.push(i);
        running.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let middle = running.len() / 2;
        if running.len() % 2 == 0 {
            medians.push((running[middle - 1] + running[middle]) / 2.0);
        } else {
            medians.push(running[middle]);
        }
    }

    medians
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_033() {
        assert_eq!(
            problem_033(vec![2.0, 1.0, 5.0, 7.0, 2.0, 0.0, 5.0]),
            vec![2.0, 1.5, 2.0, 3.5, 2.0, 2.0, 2.0]
        );
    }
}
