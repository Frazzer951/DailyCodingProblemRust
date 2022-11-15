/* MEDIUM
Given a string of parentheses, write a function to compute the minimum number of
parentheses to be removed to make the string valid (i.e. each open parenthesis
is eventually closed).

For example, given the string "()())()", you should return 1. Given the string
")(", you should return 2, since we must remove all of them.
*/

fn problem_086(parens: String) -> i32 {
    let mut open = 0;
    let mut count = 0;
    for c in parens.chars() {
        if c == '(' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        } else {
            count += 1;
        }
    }
    count + open
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_086() {
        assert_eq!(problem_086("()())()".to_owned()), 1);
        assert_eq!(problem_086(")(".to_owned()), 2);
    }
}
