/* EASY
Given a mapping of digits to letters (as in a phone number), and a digit string,
return all possible letters the number could represent. You can assume each
valid number in the mapping is a single digit.

For example if {“2”: [“a”, “b”, “c”], 3: [“d”, “e”, “f”], …} then “23” should
return [“ad”, “ae”, “af”, “bd”, “be”, “bf”, “cd”, “ce”, “cf"].
*/

use std::collections::HashMap;

fn possible_strings(mapping: &HashMap<char, Vec<char>>, number: String) -> Vec<String> {
    let n = number.len();

    let mut possibilities: Vec<String> = vec![];

    for index in (0..n).rev() {
        let n = number.chars().nth(index).unwrap();
        let mut new_pos = vec![];
        for c in mapping.get(&n).unwrap() {
            if possibilities.is_empty() {
                new_pos.push(c.to_string())
            } else {
                for p in &possibilities {
                    new_pos.push(c.to_string() + p);
                }
            }
        }
        possibilities = new_pos;
    }

    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_strings() {
        let map = HashMap::from([('2', vec!['a', 'b', 'c']), ('3', vec!['d', 'e', 'f'])]);
        assert_eq!(
            possible_strings(&map, "23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
