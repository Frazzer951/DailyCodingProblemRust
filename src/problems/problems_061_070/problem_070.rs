/* EASY
A number is considered perfect if its digits sum up to exactly 10.

Given a positive integer n, return the n-th perfect number.

For example, given 1, you should return 19. Given 2, you should return 28.
*/

fn next_perfect_number(mut num: i32) -> i32 {
    let mut sum_digits = 0;
    let result = num;

    while num > 0 {
        sum_digits += num % 10;
        num /= 10;
    }

    let remaining = 10 - sum_digits;

    if remaining == 0 {
        result
    } else {
        result * 10 + remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_perfect_number_1() {
        assert_eq!(next_perfect_number(1), 19);
    }

    #[test]
    fn test_next_perfect_number_2() {
        assert_eq!(next_perfect_number(2), 28);
    }

    #[test]
    fn test_next_perfect_number_11() {
        assert_eq!(next_perfect_number(11), 118);
    }

    #[test]
    fn test_next_perfect_number_18() {
        assert_eq!(next_perfect_number(18), 181);
    }

    #[test]
    fn test_next_perfect_number_19() {
        assert_eq!(next_perfect_number(19), 19);
    }
}
