/* MEDIUM
Given the mapping a = 1, b = 2, ... z = 26, and an encoded message, count the
number of ways it can be decoded.

For example, the message '111' would give 3, since it could be decoded as 'aaa',
'ka', and 'ak'.

You can assume that the messages are decodable. For example, '001' is not
allowed.
*/

fn problem_007(s: &str) -> i64 {
    if s.len() == 0 {
        return 1;
    }
    if s.chars().next().unwrap() == '0' {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }

    let mut total = 0;
    let x: i64 = s[0..2].parse().unwrap();
    if x <= 26 {
        total += problem_007(&s[2..]);
    }

    total += problem_007(&s[1..]);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_007() {
        assert_eq!(problem_007("111"), 3);
        assert_eq!(problem_007("001"), 0);
    }
}
