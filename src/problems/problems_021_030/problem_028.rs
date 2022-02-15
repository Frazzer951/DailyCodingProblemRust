/* MEDIUM
Write an algorithm to justify text. Given a sequence of words and an integer
line length k, return a list of strings which represents each line, fully
justified.

More specifically, you should have as many words as possible in each line. There
should be at least one space between each word. Pad extra spaces when necessary
so that each line has exactly length k. Spaces should be distributed as equally
as possible, with the extra spaces, if any, distributed starting from the left.

If you can only fit one word on a line, then you should pad the right-hand side
with spaces.

Each word is guaranteed not to be longer than k.

For example, given the list of words ["the", "quick", "brown", "fox", "jumps",
"over", "the", "lazy", "dog"] and k = 16, you should return the following:

["the  quick brown", # 1 extra space on the left
"fox  jumps  over", # 2 extra spaces distributed evenly
"the   lazy   dog"] # 4 extra spaces distributed evenly
*/

fn problem_028(mut words: Vec<String>, k: usize) -> Vec<String> {
    let mut padded_strings = vec![];

    while !words.is_empty() {
        let mut length = 0;
        let mut line = vec![];
        while length < k {
            if words.is_empty() {
                break;
            };
            let word = words[0].clone();
            if length + word.len() + line.len() <= k {
                line.push(word);
                words.remove(0);
                length += line[line.len() - 1].len();
            } else {
                break;
            }
        }
        length += line.len() - 1;
        let mut add_spaces = k - length;

        while add_spaces > 0 {
            for i in 0..line.len() - 1 {
                if add_spaces == 0 {
                    break;
                }
                line[i] += " ";
                add_spaces -= 1;
            }
        }

        let mut temp = String::new();
        for i in 0..line.len() {
            temp += &line[i];
            if i != line.len() - 1 {
                temp += " ";
            }
        }
        padded_strings.push(temp);
    }

    padded_strings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_028() {
        assert_eq!(
            problem_028(
                vec![
                    String::from("the"),
                    String::from("quick"),
                    String::from("brown"),
                    String::from("fox"),
                    String::from("jumps"),
                    String::from("over"),
                    String::from("the"),
                    String::from("lazy"),
                    String::from("dog")
                ],
                16
            ),
            [
                String::from("the  quick brown"),
                String::from("fox  jumps  over"),
                String::from("the   lazy   dog"),
            ]
        );
    }
}
