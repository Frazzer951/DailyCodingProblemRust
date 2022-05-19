/* MEDIUM
Implement an autocomplete system. That is, given a query string s and a set of
all possible query strings, return all strings in the set that have s as a
prefix.

For example, given the query string de and the set of strings [dog, deer, deal],
return [deer, deal].

Hint: Try preprocessing the dictionary into a more efficient data structure to
speed up queries.
*/

fn problem_011(qstr: &str, qstrs: Vec<&str>) -> Vec<String> {
    let mut return_strs: Vec<String> = vec![];

    for word in qstrs {
        if qstr == &word[..qstr.len()] {
            return_strs.push(String::from(word));
        }
    }

    return_strs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_011() {
        assert_eq!(problem_011("de", vec! {"dog", "deer", "deal"}),
                   vec! {String::from("deer"), String::from("deal")});
    }
}
