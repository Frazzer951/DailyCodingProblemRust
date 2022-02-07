// NOT DONE

/* EASY
What does the below code snippet print out? How can we fix the anonymous
functions to behave as we'd expect?

functions = []
for i in range(10):
    functions.append(lambda : i)

for f in functions:
    print(f())
*/

fn problem_091() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_091() {
        assert_eq!(problem_091(), 1);
    }
}
