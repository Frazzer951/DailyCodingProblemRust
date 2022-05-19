/* MEDIUM
Given a dictionary of words and a string made up of those words (no spaces),
return the original sentence in a list. If there is more than one possible
reconstruction, return any of them. If there is no possible reconstruction, then
return null.

For example, given the set of words 'quick', 'brown', 'the', 'fox', and the
string "thequickbrownfox", you should return ['the', 'quick', 'brown', 'fox'].

Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and the string
"bedbathandbeyond", return either ['bed', 'bath', 'and', 'beyond] or ['bedbath',
'and', 'beyond'].
*/

fn problem_022(word_set: Vec<String>, mut str: String) -> Vec<String> {
    let mut sentance = Vec::new();

    while !str.is_empty() {
        let mut word_found = false;
        for word in &word_set {
            let index = str.find(&word.to_string()).unwrap_or(str.len());
            if index == 0 {
                sentance.push(word.clone().to_string());
                str.replace_range(0..word.len(), "");
                word_found = true;
            }
        }
        if !word_found {
            break;
        }
    }

    sentance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_022() {
        assert_eq!(problem_022(vec![String::from("quick"),
                                    String::from("brown"),
                                    String::from("the"),
                                    String::from("fox")],
                               String::from("thequickbrownfox")),
                   vec![String::from("the"),
                        String::from("quick"),
                        String::from("brown"),
                        String::from("fox")]);
        assert_eq!(problem_022(vec![String::from("bed"),
                                    String::from("bath"),
                                    String::from("bedbath"),
                                    String::from("and"),
                                    String::from("beyond")],
                               String::from("bedbathandbeyond")),
                   vec![String::from("bed"),
                        String::from("bath"),
                        String::from("and"),
                        String::from("beyond")]);
    }
}
