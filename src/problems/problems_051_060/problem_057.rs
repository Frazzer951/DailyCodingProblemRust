/* MEDIUM
Given a string s and an integer k, break up the string into multiple lines such
that each line has a length of k or less. You must break it up so that words
don't break across lines. Each line has to have the maximum possible amount of
words. If there's no way to break the text up, then return null.

You can assume that there are no spaces at the ends of the string and that there
is exactly one space between each word.

For example, given the string "the quick brown fox jumps over the lazy dog" and
k = 10, you should return: ["the quick", "brown fox", "jumps over", "the lazy",
"dog"]. No string in the list has a length of more than 10.
*/

fn problem_057(s: String, k: usize) -> Vec<String> {
    let mut words: Vec<&str> = s.split(' ').collect();
    let mut output = vec![];

    while !words.is_empty() {
        let mut cur = String::new();
        let mut cur_len = 0;
        while cur.len() < k && !words.is_empty() {
            let next_word = words[0];
            if cur_len == 0 {
                cur.push_str(next_word);
                cur_len += next_word.len();
                words.remove(0);
            } else if cur_len + next_word.len() < k {
                cur.push(' ');
                cur.push_str(next_word);
                cur_len += next_word.len() + 1;
                words.remove(0);
            } else {
                break;
            }
        }
        output.push(cur.clone());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_057() {
        assert_eq!(
            problem_057("the quick brown fox jumps over the lazy dog".to_string(), 10),
            vec![
                "the quick".to_string(),
                "brown fox".to_string(),
                "jumps over".to_string(),
                "the lazy".to_string(),
                "dog".to_string()
            ]
        );
    }
}
