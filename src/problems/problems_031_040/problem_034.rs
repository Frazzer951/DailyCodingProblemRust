/* MEDIUM
Given a string, find the palindrome that can be made by inserting the fewest
number of characters as possible anywhere in the word. If there is more than one
palindrome of minimum length that can be made, return the lexicographically
earliest one (the first one alphabetically).

For example, given the string "race", you should return "ecarace", since we can
add three letters to it (which is the smallest amount to make a palindrome).
There are seven other palindromes that can be made from "race" by adding three
letters, but "ecarace" comes first alphabetically.

As another example, given the string "google", you should return "elgoogle".
*/

use std::cmp::min;

fn is_palindrome(str: &str) -> bool {
    let reverse = str.chars().rev().collect::<String>();
    str == reverse
}

fn create_palindrome(str: &str) -> String {
    if is_palindrome(str) {
        return String::from(str);
    }

    if str.chars().next().unwrap() == str.chars().nth(str.len() - 1).unwrap() {
        let mut ret = String::from(str.chars().next().unwrap());
        ret.push_str(&create_palindrome(&str[1..str.len() - 1]));
        ret.push(str.chars().nth(str.len() - 1).unwrap());
        return ret;
    }

    let mut one = String::from(str.chars().next().unwrap());
    one.push_str(&create_palindrome(&str[1..]));
    one.push(str.chars().next().unwrap());

    let mut two = String::from(str.chars().nth(str.len() - 1).unwrap());
    two.push_str(&create_palindrome(&str[..str.len() - 1]));
    two.push(str.chars().nth(str.len() - 1).unwrap());

    if one.len() < two.len() {
        return one;
    }
    if two.len() < one.len() {
        return two;
    }

    min(one, two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_034() {
        assert_eq!(create_palindrome("race"), String::from("ecarace"));
        assert_eq!(create_palindrome("google"), String::from("elgoogle"));
        assert_eq!(create_palindrome("a"), String::from("a"));
        assert_eq!(create_palindrome("aa"), String::from("aa"));
        assert_eq!(create_palindrome("alsa"), String::from("alsla"));
        assert_eq!(create_palindrome("abb"), String::from("abba"));
    }
}
