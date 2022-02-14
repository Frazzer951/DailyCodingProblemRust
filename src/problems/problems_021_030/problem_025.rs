/* HARD
Implement regular expression matching with the following special characters:

 * . (period) which matches any single character
 * * (asterisk) which matches zero or more of the preceding element

That is, implement a function that takes in a string and a valid regular
expression and returns whether or not the string matches the regular expression.

For example, given the regular expression "ra." and the string "ray", your
function should return true. The same regular expression on the string "raymond"
should return false.

Given the regular expression ".*at" and the string "chat", your function should
return true. The same regular expression on the string "chats" should return
false.
*/

fn problem_025(str: String, exp: String) -> bool {
    let mut str_index = 0;

    for i in 0..exp.len() {
        let cur = str.chars().nth(i).unwrap();
        match exp.chars().nth(i).unwrap() {
            '.' => str_index += 1,
            '*' => {
                if exp.chars().nth(i + 1).unwrap() == cur {
                    break;
                }
                while cur == str.chars().nth(str_index).unwrap() {
                    str_index += 1;
                }
            }
            _ => {
                if exp.chars().nth(i).unwrap() != cur {
                    return false;
                }
                str_index += 1;
            }
        }
    }

    str_index == str.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_problem_025() {
        assert!(problem_025(String::from("ray"), String::from("ra.")));
        assert!(!problem_025(String::from("raymond"), String::from("ra.")));
        assert!(problem_025(String::from("chat"), String::from(".*at")));
        assert!(!problem_025(String::from("chats"), String::from(".*at")));
    }
}
