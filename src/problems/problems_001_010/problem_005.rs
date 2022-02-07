// NOT DONE

/* MEDIUM
cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and
last element of that pair. For example, car(cons(3, 4)) returns 3, and
cdr(cons(3, 4)) returns 4.

Given this implementation of cons:

def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair


Implement car and cdr.
*/

fn problem_005() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_problem_005() {
        assert_eq!(2 + 2, 4);
    }
}
