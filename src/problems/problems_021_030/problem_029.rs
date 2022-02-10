// NOT DONE

/* EASY
Run-length encoding is a fast and simple method of encoding strings. The basic
idea is to represent repeated successive characters as a single count and
character. For example, the string "AAAABBBCCDAA" would be encoded as
"4A3B2C1D2A".

Implement run-length encoding and decoding. You can assume the string to be
encoded have no digits and consists solely of alphabetic characters. You can
assume the string to be decoded is valid.
*/

fn count_char(str: &String, c: char) -> usize {
    let mut count = 0;
    for i in str.chars() {
        if i == c {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn run_length_encode(mut str: String) -> String {
    let mut result = String::new();

    while !str.is_empty() {
        let c = str.chars().next().unwrap();
        let count = count_char(&str, c);
        str.replace_range(0..count, "");
        result.push_str(&format!("{}{}", count, c));
    }

    result
}

fn run_length_decode(mut str: String) -> String {
    let mut result = String::new();


    while !str.is_empty() {
        let mut chars = str.chars();
        let x = chars.next().unwrap();
        let c = chars.next().unwrap();
        let x = x.to_digit(10).unwrap();
        for _ in 0..x {
            result.push(c);
        }
        str.replace_range(0..2, "");
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_029() {
        assert_eq!(run_length_encode(String::from("AAAABBBCCDAA")), String::from("4A3B2C1D2A"));
        assert_eq!(run_length_decode(String::from("4A3B2C1D2A")), String::from("AAAABBBCCDAA"));
    }
}
