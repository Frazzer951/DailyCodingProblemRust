// NOT DONE

/* HARD
A Boolean formula can be said to be satisfiable if there is a way to assign
truth values to each variable such that the entire formula evaluates to true.

For example, suppose we have the following formula, where the symbol ¬ is used
to denote negation:

(¬c OR b) AND (b OR c) AND (¬b OR c) AND (¬c OR ¬a)

One way to satisfy this formula would be to let a = False, b = True, and c =
True.

This type of formula, with AND statements joining tuples containing exactly one
OR, is known as 2-CNF.

Given a 2-CNF formula, find a way to assign truth values to satisfy it, or
return False if this is impossible.
*/

fn problem_330() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_problem_330() {
        assert_eq!(problem_330(), 1);
    }
}
