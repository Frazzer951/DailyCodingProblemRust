// NOT DONE

/* EASY
Write a function to flatten a nested dictionary. Namespace the keys with a
period.

For example, given the following dictionary:

{
    "key": 3,
    "foo": {
        "a": 5,
        "bar": {
            "baz": 8
        }
    }
}


it should become:

{
    "key": 3,
    "foo.a": 5,
    "foo.bar.baz": 8
}


You can assume keys do not contain dots in them, i.e. no clobbering will occur.
*/

fn problem_173() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_173() {
        assert_eq!(problem_173(), 1);
    }
}
