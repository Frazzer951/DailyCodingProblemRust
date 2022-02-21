// NOT DONE

/* MEDIUM
Implement integer exponentiation. That is, implement the pow(x, y) function,
where x and y are integers and returns x^y.

Do this faster than the naive method of repeated multiplication.

For example, pow(2, 10) should return 1024.
*/

fn problem_061(x: i64, y: i64) -> i64 {
    if y == 0 {
        1
    } else if y == 1 {
        x
    } else if y % 2 == 0 {
        problem_061(x * x, y / 2)
    } else {
        x * problem_061(x * x, y / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_061() {
        assert_eq!(problem_061(2, 10), 1024);
    }
}
