// NOT DONE

/* MEDIUM
Implement the singleton pattern with a twist. First, instead of storing one
instance, store two instances. And in every even call of getInstance(), return
the first instance and in every odd call of getInstance(), return the second
instance.
*/

fn problem_120() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_120() {
        assert_eq!(problem_120(), 1);
    }
}
