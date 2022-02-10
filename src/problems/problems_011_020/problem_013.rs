/* HARD
Given an integer k and a string s, find the length of the longest substring that
contains at most k distinct characters.

For example, given s = "abcba" and k = 2, the longest substring with k distinct
characters is "bcb".
*/

use std::collections::HashMap;

fn problem_013(s: String, k: usize) -> usize {
    if k == 0 {
        return 0;
    }
    let mut h: HashMap<char, usize> = HashMap::new();
    let mut bounds = (0, 0);
    let mut max_len = 0;
    let mut new_lower_bound;
    for i in 0..s.len() {
        h.insert(s.chars().nth(i).unwrap(), i);
        if h.len() <= k {
            new_lower_bound = bounds.0;
        } else {
            let key_to_pop = h.iter().min_by_key(|(_, v)| *v).unwrap().0;
            let key_to_pop = key_to_pop.clone();
            new_lower_bound = h[&key_to_pop] + 1;
            h.remove(&key_to_pop);
        }
        bounds = (new_lower_bound, i + 1);
        max_len = max_len.max(bounds.1 - bounds.0);
    }
    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_013() {
        assert_eq!(problem_013(String::from("abcba"), 2), 3);
    }
}
