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

fn is_match(str: String, exp: String) -> bool {
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
            },
            _ => {
                if exp.chars().nth(i).unwrap() != cur {
                    return false;
                }
                str_index += 1;
            },
        }
    }

    str_index == str.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match_1() {
        assert!(is_match(String::from("ray"), String::from("ra.")));
        assert!(!is_match(String::from("raymond"), String::from("ra.")));
    }

    #[test]
    fn test_is_match_2() {
        assert!(is_match(String::from("chat"), String::from(".*at")));
        assert!(!is_match(String::from("chats"), String::from(".*at")));
    }

    #[test]
    fn test_is_match_3() {
        assert!(!is_match(String::from("cat"), String::from("hat")));
    }
}
