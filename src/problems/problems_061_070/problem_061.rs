/* MEDIUM
Implement integer exponentiation. That is, implement the pow(x, y) function,
where x and y are integers and returns x^y.

Do this faster than the naive method of repeated multiplication.

For example, pow(2, 10) should return 1024.
*/

fn power(x: i32, y: u32) -> i32 {
    if y == 0 {
        return 1;
    }
    if y == 1 {
        return x;
    }
    if y % 2 == 0 {
        return power(x * x, y / 2);
    }
    x * power(x * x, y / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(5, 2), 25);
        assert_eq!(power(7, 5), 16807);
    }
}
