// NOT DONE

/* MEDIUM
What will this code print out?

def make_functions():
    flist = []

    for i in [1, 2, 3]:
        def print_i():
            print(i)
        flist.append(print_i)

    return flist

functions = make_functions()
for f in functions:
    f()


How can we make it print out what we apparently want?
*/

fn problem_188() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_problem_188() {
        assert_eq!(2 + 2, 4);
    }
}
