/* EASY
Given a string of round, curly, and square open and closing brackets, return
whether the brackets are balanced (well-formed).

For example, given the string "([])[]({})", you should return true.

Given the string "([)]" or "((()", you should return false.
*/

use std::collections::VecDeque;

fn problem_027(string: &str) -> bool {
    let mut next_bracket = VecDeque::new();

    for c in string.chars() {
        match c {
            '(' => next_bracket.push_back(')'),
            '[' => next_bracket.push_back(']'),
            '{' => next_bracket.push_back('}'),
            ')' | ']' | '}' => {
                let next_bracket_char = next_bracket.pop_back();
                if next_bracket_char.is_none() {
                    return false;
                }
                if c != next_bracket_char.unwrap() {
                    return false;
                }
            }
            _ => {
                panic!("Unexpected character: {}", c);
            }
        }
    }

    next_bracket.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_027() {
        assert!(problem_027("([])[]({})"));
        assert!(!problem_027("([)]"));
        assert!(!problem_027("((()"));
    }
}
